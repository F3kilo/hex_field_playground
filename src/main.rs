use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use settings_path::*;
use slog::{crit, error, info, trace, warn, Logger};
use sloggers::{file::FileLoggerBuilder, types::TimeZone, Build};
use std::env::VarError;
use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use tinyfiledialogs::*;
mod error;
use error::init::InitError;
use error::log_init::LogInitError;

fn main() {
    let (logger) = init().unwrap_or_else(|e| {
        let message = format!("Initialization error occurred: {}", e);
        show_error_message("Initialization error", message.as_str());
        panic!(message);
    });
}

fn show_error_message(title: &str, message: &str) {
    message_box_ok(title, message, MessageBoxIcon::Error);
}

/// Basis structures initialization
fn init() -> Result<(Logger,), InitError> {
    let mut save_path = default_settings_path()?;
    save_path.push("HexFieldPlayground");

    let logger = init_logger(save_path)?;
    info!(
        logger,
        "=============== START NEW SESSION ==============="
    );
    trace!(logger, "Logger initilized");

    Ok((logger,))
}

/// Logger initialization
fn init_logger(save_path: PathBuf) -> Result<Logger, LogInitError> {
    let log_dir = save_path.join("logs");
    let log_path = save_path.join("logs\\log");
    println!("d: {:?}", log_dir);
    println!("p: {:?}", log_path);
    std::fs::create_dir_all(log_dir.clone())?;
    FileLoggerBuilder::new(log_path)
        .timezone(TimeZone::Local)
        .rotate_size(10 * 2u64.pow(20))
        .build()
        .map_err(|e| e.into())
}

struct HexFieldPlayground {}

impl HexFieldPlayground {
    fn new() -> Self {
        Self {}
    }
    //
    // fn init() {}
    //
    // fn run() {}
}

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
