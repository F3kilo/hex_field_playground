use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop};
use winit::window::{Window, WindowBuilder};

use settings_path::*;
use slog::{info, trace, Logger};
use sloggers::{file::FileLoggerBuilder, types::TimeZone, Build};
use std::path::PathBuf;
use tinyfiledialogs::*;

mod error;

use error::init::InitError;
use error::log_init::LogInitError;
use sloggers::types::Severity;

fn main() {
    let (logger, _event_loop, _window) = init().unwrap_or_else(|e| {
        let message = format!("Initialization error occurred: {}", e);
        show_error_message("Initialization error", message.as_str());
        panic!(message);
    });

    trace!(logger, "Initialization done");
}

fn show_error_message(title: &str, message: &str) {
    message_box_ok(title, message, MessageBoxIcon::Error);
}

/// Basis structures initialization
fn init() -> Result<(Logger, EventLoop<()>, Window), InitError> {
    let mut save_path = default_settings_path()?;
    save_path.push("HexFieldPlayground");

    // Init logger
    let logger = init_logger(save_path)?;
    info!(logger, "=============== START NEW SESSION ===============");
    trace!(logger, "Logger initilized");

    // Init event loop and window
    let event_loop = EventLoop::new();
    trace!(logger, "Event loop initialized");

    let window_builder = WindowBuilder::default()
        .with_title("HexFieldPlayground")
        .with_inner_size(PhysicalSize::new(1920, 1080));
    let window = window_builder
        .build(&event_loop)?;
    trace!(logger, "Window initialized");

    Ok((logger, event_loop, window))
}

/// Logger initialization
fn init_logger(save_path: PathBuf) -> Result<Logger, LogInitError> {
    let log_dir = save_path.join("logs");
    let log_path = save_path.join("logs\\log");
    std::fs::create_dir_all(log_dir)?;
    FileLoggerBuilder::new(log_path)
        .timezone(TimeZone::Local)
        .rotate_size(10 * 2u64.pow(20))
        .level(Severity::Trace)
        .build()
        .map_err(|e| e.into())
}

// struct HexFieldPlayground {}
//
// impl HexFieldPlayground {
//     fn new() -> Self {
//         Self {}
//     }
//
//     fn init() {}
//
//     fn run() {}
// }

// fn run() {
//     let logger = init_logger();
//
//     let event_loop = EventLoop::new();
//     let window_builder = WindowBuilder::default()
//         .with_title("Test window")
//         .with_inner_size(PhysicalSize::new(1920, 1080));
//     let window = window_builder
//         .build(&event_loop)
//         .expect("Can't create window");
//     println!("Run");
//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Poll;
//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => {
//                 println!("Close event");
//                 *control_flow = ControlFlow::Exit;
//             }
//             Event::DeviceEvent(_, event) => {}
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//                 println!("request_redraw called");
//             }
//             Event::RedrawRequested(_) => {
//                 println!("RedrawRequested");
//             }
//             _ => (),
//         };
//     });
// }
