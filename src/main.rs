mod app;
mod error;

use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use settings_path::*;
#[macro_use]
extern crate slog;
use slog::Logger;
use sloggers::{file::FileLoggerBuilder, types::TimeZone, Build};
use std::path::PathBuf;
use tinyfiledialogs::*;

use error::init::InitError;
use error::log_init::LogInitError;
use sloggers::types::Severity;
use winit::error::OsError;

use crate::app::App;
use std::convert::TryInto;
use std::sync::mpsc::channel;

fn main() {
    let (logger, event_loop, _window) = base_init().unwrap_or_else(|e| {
        let message = format!("Initialization error occurred: {}", e);
        show_error_message("Initialization error", message.as_str());
        panic!(message);
    });

    let (event_tx, event_rx) = channel();
    let _app = App::new(event_rx);

    info!(logger, "Initialization done");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(e) = event.try_into() {
            event_tx.send(e).unwrap_or_else(|_| {
                error!(logger, "Application disconnected. Exitting...");
                *control_flow = ControlFlow::Exit;
            });
        }
    });
}

fn show_error_message(title: &str, message: &str) {
    message_box_ok(title, message, MessageBoxIcon::Error);
}

/// Basis structures initialization
fn base_init() -> Result<(Logger, EventLoop<()>, Window), InitError> {
    let mut save_path = default_settings_path()?;
    save_path.push("HexFieldPlayground");

    // Init logger
    let logger = init_logger(save_path)?;
    info!(logger, "=============== START NEW SESSION ===============");
    trace!(logger, "Logger initilized");

    // Init event loop
    let event_loop = EventLoop::new();
    trace!(logger, "Event loop initialized");

    // Init window
    let window = init_window(&event_loop)?;
    trace!(logger, "Window initialized");

    Ok((logger, event_loop, window))
}

/// Window initialization
fn init_window(event_loop: &EventLoop<()>) -> Result<Window, OsError> {
    let window_builder = WindowBuilder::default()
        .with_title("HexFieldPlayground")
        .with_inner_size(PhysicalSize::new(800, 600));
    let window = window_builder.build(&event_loop)?;
    Ok(window)
}

/// Logger initialization
fn init_logger(save_path: PathBuf) -> Result<Logger, LogInitError> {
    let log_dir = save_path.join("logs");
    let log_path = save_path.join("logs\\log");
    std::fs::create_dir_all(log_dir)?;
    let logger = FileLoggerBuilder::new(log_path)
        .timezone(TimeZone::Local)
        .rotate_size(10 * 2u64.pow(20))
        .level(Severity::Trace)
        .build()?;
    Ok(logger)
}
