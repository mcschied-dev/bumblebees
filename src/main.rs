//! BumbleBees - Main entry point
//!
//! Initializes the game engine, loads resources, and starts the game loop.

use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::path::Path;

use ten::{init_logger, MainState, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Main entry point for the BumbleBees game.
///
/// Initializes logging, creates the game context, mounts resources,
/// and starts the game event loop.
fn main() -> GameResult {
    // Initialize logging system
    if let Err(e) = init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log::info!("Starting BumbleBees game");
    log::debug!("Screen dimensions: {}x{}", SCREEN_WIDTH, SCREEN_HEIGHT);

    // Determine resource path based on project root
    let resources_dir = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));
    log::debug!("Resource directory: {}", resources_dir);

    // Create context
    let (mut ctx, event_loop) = ContextBuilder::new("bumblebees", "mcschied")
        .window_setup(ggez::conf::WindowSetup::default().title("BumbleBees"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    log::info!("Game context created successfully");

    // Mount resource directory
    ctx.fs.mount(Path::new(&resources_dir), true);
    log::info!("Resources mounted from: {}", resources_dir);

    // Create game state
    log::debug!("Initializing game state");
    let state = MainState::new(&mut ctx)?;
    log::info!("Game state initialized, starting event loop");

    // Run game
    event::run(ctx, event_loop, state)
}
