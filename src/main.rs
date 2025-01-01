use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Rect, Text, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::input::keyboard::is_key_pressed;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 300.0;
const BULLET_SPEED: f32 = 500.0;
const INITIAL_ENEMY_SPEED: f32 = 50.0;
const DEFENDER_LINE: f32 = 100.0; // Linie, die der Verteidiger hält

struct Bullet {
    x: f32,
    y: f32,
}

struct Enemy {
    x: f32,
    y: f32,
}

enum GameState {
    Playing,
    GameOver,
}

struct MainState {
    player_x: f32,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    enemy_direction: f32,
    enemy_speed: f32,
    wave: u32,
    state: GameState,
    shoot_sink: Sink,
    hit_sink: Sink,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Audio initialisieren
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let shoot_sink = Sink::try_new(&stream_handle).unwrap();
        let hit_sink = Sink::try_new(&stream_handle).unwrap();

        // Schuss-Sound laden
        let shoot_file = BufReader::new(File::open("resources/shoot.ogg")
        .expect("Schuss-Sounddatei 'resources/shoot.ogg' konnte nicht gefunden werden."));
        let shoot_source = rodio::Decoder::new(shoot_file).unwrap();
        shoot_sink.append(shoot_source);

        // Treffer-Sound laden
        let hit_file = BufReader::new(File::open("resources/hit.ogg")
        .expect("Treffer-Sounddatei 'resources/hit.ogg' konnte nicht gefunden werden."));
        let hit_source = rodio::Decoder::new(hit_file).unwrap();
        hit_sink.append(hit_source);

        Ok(MainState {
            player_x: SCREEN_WIDTH / 2.0,
            bullets: Vec::new(),
            enemies: MainState::generate_enemies(1),
            enemy_direction: 1.0,
            enemy_speed: INITIAL_ENEMY_SPEED,
            wave: 1,
            state: GameState::Playing,
            shoot_sink,
            hit_sink,
        })
    }

    fn reset(&mut self) {
        self.player_x = SCREEN_WIDTH / 2.0;
        self.bullets.clear();
        self.enemies = MainState::generate_enemies(1);
        self.enemy_direction = 1.0;
        self.enemy_speed = INITIAL_ENEMY_SPEED;
        self.wave = 1;
        self.state = GameState::Playing;
    }

    fn generate_enemies(wave: u32) -> Vec<Enemy> {
        let mut enemies = Vec::new();
        for i in 0..10 {
            for j in 0..3 {
                enemies.push(Enemy {
                    x: 50.0 + i as f32 * 60.0,
                    y: 50.0 + j as f32 * 50.0,
                });
            }
        }
        enemies
    }

    fn shoot(&mut self) {
        if matches!(self.state, GameState::Playing) {
            self.bullets.push(Bullet {
                x: self.player_x,
                y: SCREEN_HEIGHT - 50.0,
            });

            // Schuss-Sound abspielen
            self.shoot_sink.stop();
            let shoot_file = BufReader::new(File::open("resources/shoot.ogg").unwrap());
            let shoot_source = rodio::Decoder::new(shoot_file).unwrap();
            self.shoot_sink.append(shoot_source);
            self.shoot_sink.play();
        }
    }

    fn next_wave(&mut self) {
        self.wave += 1;
        self.enemies = MainState::generate_enemies(self.wave);
        self.enemy_speed += 20.0; // Erhöhe die Geschwindigkeit der Gegner
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if matches!(self.state, GameState::GameOver) {
            return Ok(()); // Keine Aktualisierung im Game-Over-Zustand
        }

        let dt = ctx.time.delta().as_secs_f32();

        // Bewegung des Spielers basierend auf Eingaben
        if is_key_pressed(ctx, KeyCode::Left) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if is_key_pressed(ctx, KeyCode::Right) {
            self.player_x += PLAYER_SPEED * dt;
        }

        // Begrenzung des Spielers auf den Bildschirmbereich
        self.player_x = self.player_x.clamp(25.0, SCREEN_WIDTH - 25.0);

        // Bewegung der Gegner
        for enemy in &mut self.enemies {
            enemy.x += self.enemy_direction * self.enemy_speed * dt;
        }

        // Richtung ändern und nach unten bewegen, wenn Gegner den Rand erreichen
        if self
            .enemies
            .iter()
            .any(|e| e.x <= 25.0 || e.x >= SCREEN_WIDTH - 25.0)
        {
            self.enemy_direction *= -1.0;
            for enemy in &mut self.enemies {
                enemy.y += 20.0; // Gegner bewegen sich nach unten
            }
        }

        // Bewegung der Schüsse
        for bullet in &mut self.bullets {
            bullet.y -= BULLET_SPEED * dt;
        }
        self.bullets.retain(|b| b.y > 0.0); // Entferne Schüsse außerhalb des Bildschirms

        // Kollisionserkennung
        let mut hit_detected = false;
        self.enemies.retain(|enemy| {
            if self.bullets.iter().any(|bullet| {
                let distance = ((enemy.x - bullet.x).powi(2) + (enemy.y - bullet.y).powi(2)).sqrt();
                distance < 20.0 // Trefferbereich
            }) {
                hit_detected = true;
                false
            } else {
                true
            }
        });

        if hit_detected {
            // Treffer-Sound abspielen
            self.hit_sink.stop();
            let hit_file = BufReader::new(File::open("resources/hit.ogg").unwrap());
            let hit_source = rodio::Decoder::new(hit_file).unwrap();
            self.hit_sink.append(hit_source);
            self.hit_sink.play();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Canvas für das Zeichnen erstellen
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Spieler zeichnen
        let player_rect = Rect::new(self.player_x - 25.0, SCREEN_HEIGHT - 50.0, 50.0, 20.0);
        let player_color = Color::from_rgb(0, 255, 0);
        let player_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), player_rect, player_color)?;
        canvas.draw(&player_mesh, graphics::DrawParam::default());

        // Schüsse zeichnen
        for bullet in &self.bullets {
            let bullet_rect = Rect::new(bullet.x - 5.0, bullet.y - 10.0, 10.0, 20.0);
            let bullet_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bullet_rect, Color::WHITE)?;
            canvas.draw(&bullet_mesh, graphics::DrawParam::default());
        }

        // Gegner zeichnen
        for enemy in &self.enemies {
            let enemy_rect = Rect::new(enemy.x - 20.0, enemy.y - 20.0, 40.0, 40.0);
            let enemy_color = Color::from_rgb(255, 0, 0);
            let enemy_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), enemy_rect, enemy_color)?;
            canvas.draw(&enemy_mesh, graphics::DrawParam::default());
        }

        // Canvas abschließen
        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(KeyCode::Space) = input.keycode {
            self.shoot();
        }
        if let Some(KeyCode::R) = input.keycode {
            self.reset();
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("space_invaders", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Space Invaders"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let mut state = MainState::new(&mut ctx)?; // ctx ist jetzt mutable
    event::run(ctx, event_loop, state)
}