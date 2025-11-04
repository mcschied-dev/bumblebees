//! Logging configuration and initialization.
//!
//! This module sets up the logging system using the `fern` crate to write
//! logs to both the console and a debug.log file.

use chrono::Local;
use log::LevelFilter;
use std::io;

/// Initialize the logging system.
///
/// Sets up logging to write to both stdout and a debug.log file with timestamps.
/// Log levels are configured as follows:
/// - Console: INFO and above
/// - File: DEBUG and above
///
/// # Errors
///
/// Returns an error if the logger cannot be initialized.
pub fn init() -> Result<(), fern::InitError> {
    // Create base configuration
    let base_config = fern::Dispatch::new();

    // Console output configuration (INFO and above)
    let console_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout());

    // File output configuration (DEBUG and above)
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file("debug.log")?);

    // Combine configurations
    base_config
        .chain(console_config)
        .chain(file_config)
        .apply()?;

    log::info!("Logging system initialized");
    log::debug!("Debug logging enabled to debug.log");

    Ok(())
}
