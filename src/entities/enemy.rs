//! Enemy entity implementation.

use crate::constants::{DEFENDER_LINE, SCREEN_HEIGHT};

/// Enemy type determines behavior, appearance, health, and point value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    /// Standard enemy - 1 hit, normal speed, 10 points
    Standard,
    /// Fast enemy - 1 hit, 1.5x speed, 20 points, moves faster
    Fast,
    /// Tank enemy - 3 hits, 0.7x speed, 50 points, requires multiple hits
    Tank,
    /// Swooper enemy - 1 hit, 30 points, can dive at player (future)
    Swooper,
}

impl EnemyType {
    /// Get the maximum health for this enemy type.
    #[must_use]
    pub const fn max_health(self) -> u32 {
        match self {
            Self::Standard => 1,
            Self::Fast => 1,
            Self::Tank => 3,
            Self::Swooper => 1,
        }
    }

    /// Get the speed multiplier for this enemy type.
    #[must_use]
    pub const fn speed_multiplier(self) -> f32 {
        match self {
            Self::Standard => 1.0,
            Self::Fast => 1.5,
            Self::Tank => 0.7,
            Self::Swooper => 1.0,
        }
    }

    /// Get the point value for destroying this enemy type.
    #[must_use]
    pub const fn points(self) -> u32 {
        match self {
            Self::Standard => 10,
            Self::Fast => 20,
            Self::Tank => 50,
            Self::Swooper => 30,
        }
    }
}

/// Represents an enemy in the game.
///
/// Enemies move horizontally across the screen in their own direction,
/// drop down when they hit the edge, and trigger game over if they reach the defender line.
#[derive(Debug, Clone)]
pub struct Enemy {
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
    /// Movement direction (1.0 = right, -1.0 = left)
    pub direction: f32,
    /// Enemy type determines behavior and appearance
    pub enemy_type: EnemyType,
    /// Current health (when 0, enemy is destroyed)
    pub health: u32,
}

impl Enemy {
    /// Create a new enemy at the specified position with a movement direction and type.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial X coordinate
    /// * `y` - Initial Y coordinate
    /// * `direction` - Movement direction (1.0 = right, -1.0 = left)
    /// * `enemy_type` - Type of enemy (Standard, Fast, Tank, Swooper)
    #[must_use]
    pub fn new(x: f32, y: f32, direction: f32, enemy_type: EnemyType) -> Self {
        let health = enemy_type.max_health();
        Self {
            x,
            y,
            direction,
            enemy_type,
            health,
        }
    }

    /// Update enemy position based on speed and delta time.
    /// Uses the enemy's own direction and type speed multiplier for movement.
    ///
    /// # Arguments
    ///
    /// * `base_speed` - Base movement speed in pixels per second
    /// * `dt` - Delta time in seconds
    pub fn update(&mut self, base_speed: f32, dt: f32) {
        let speed = base_speed * self.enemy_type.speed_multiplier();
        self.x += self.direction * speed * dt;
    }

    /// Damage the enemy by reducing health by 1.
    /// Returns true if enemy is destroyed (health reaches 0).
    #[must_use]
    pub fn take_damage(&mut self) -> bool {
        if self.health > 0 {
            self.health -= 1;
        }
        self.health == 0
    }

    /// Check if enemy is destroyed (health is 0).
    #[must_use]
    pub const fn is_destroyed(&self) -> bool {
        self.health == 0
    }

    /// Check if enemy has breached the defender line (game over condition).
    #[must_use]
    pub fn has_breached_defender_line(&self) -> bool {
        self.y > SCREEN_HEIGHT - DEFENDER_LINE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defender_line_breach() {
        let enemy = Enemy::new(
            100.0,
            SCREEN_HEIGHT - DEFENDER_LINE + 10.0,
            1.0,
            EnemyType::Standard,
        );
        assert!(enemy.has_breached_defender_line());
    }

    #[test]
    fn test_no_defender_line_breach() {
        let enemy = Enemy::new(
            100.0,
            SCREEN_HEIGHT - DEFENDER_LINE - 10.0,
            1.0,
            EnemyType::Standard,
        );
        assert!(!enemy.has_breached_defender_line());
    }

    #[test]
    fn test_enemy_types() {
        let standard = Enemy::new(100.0, 100.0, 1.0, EnemyType::Standard);
        let fast = Enemy::new(100.0, 100.0, 1.0, EnemyType::Fast);
        let tank = Enemy::new(100.0, 100.0, 1.0, EnemyType::Tank);
        let swooper = Enemy::new(100.0, 100.0, 1.0, EnemyType::Swooper);

        assert_eq!(standard.health, 1);
        assert_eq!(fast.health, 1);
        assert_eq!(tank.health, 3);
        assert_eq!(swooper.health, 1);
    }

    #[test]
    fn test_take_damage() {
        let mut enemy = Enemy::new(100.0, 100.0, 1.0, EnemyType::Tank);
        assert_eq!(enemy.health, 3);
        assert!(!enemy.is_destroyed());

        assert!(!enemy.take_damage()); // 3 -> 2, not destroyed
        assert_eq!(enemy.health, 2);

        assert!(!enemy.take_damage()); // 2 -> 1, not destroyed
        assert_eq!(enemy.health, 1);

        assert!(enemy.take_damage()); // 1 -> 0, destroyed!
        assert_eq!(enemy.health, 0);
        assert!(enemy.is_destroyed());
    }

    #[test]
    fn test_speed_multipliers() {
        assert_eq!(EnemyType::Standard.speed_multiplier(), 1.0);
        assert_eq!(EnemyType::Fast.speed_multiplier(), 1.5);
        assert_eq!(EnemyType::Tank.speed_multiplier(), 0.7);
        assert_eq!(EnemyType::Swooper.speed_multiplier(), 1.0);
    }

    #[test]
    fn test_points() {
        assert_eq!(EnemyType::Standard.points(), 10);
        assert_eq!(EnemyType::Fast.points(), 20);
        assert_eq!(EnemyType::Tank.points(), 50);
        assert_eq!(EnemyType::Swooper.points(), 30);
    }
}
