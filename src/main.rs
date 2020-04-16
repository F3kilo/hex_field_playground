mod app;
mod counter_model;
mod counter_text_present;
mod error;
mod term_render;

use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
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

use crate::app::{App, Status};
use std::convert::TryInto;
use std::sync::mpsc::{channel, Sender};

use crate::app::event::ApplicationEvent;
use crate::counter_text_present::CounterTextPresent;
use crate::term_render::TermRender;
use counter_model::CounterModel;
use winit::event::Event::UserEvent;

pub enum EventLoopNotice {
    AppFinished,
}

#[allow(unreachable_code)]
fn main() {
    let (logger, event_loop, _window) = base_init().unwrap_or_else(|e| {
        let message = format!("Initialization error occurred: {}", e);
        show_error_message("Initialization error", message.as_str());
        panic!(message);
    });

    let event_loop_proxy = event_loop.create_proxy();
    let app_event_tx = run_app(logger.clone(), event_loop_proxy);

    info!(logger, "Initialization done");

    event_loop.run(move |event, _, control_flow| {
        if let Event::NewEvents(StartCause::Init) = event {
            *control_flow = ControlFlow::Wait;
        }

        if let UserEvent(ApplicationEvent::Finished) = event {
            trace!(logger, "Exitting from event loop.");
            *control_flow = ControlFlow::Exit;
        }

        if let Ok(input_event) = event.try_into() {
            app_event_tx.send(input_event).unwrap_or_else(|_| {
                error!(logger, "Application disconnected. Exitting...");
                *control_flow = ControlFlow::Exit;
            })
        }
    });
}

fn run_app(
    logger: Logger,
    event_loop_proxy: EventLoopProxy<ApplicationEvent>,
) -> Sender<app::event::Event> {
    let app_thread_logger = logger.clone();
    let counter = CounterModel::new();
    let present = CounterTextPresent;
    let render = TermRender::new();
    let (event_tx, event_rx) = channel();

    let app = App::new(event_rx, logger.clone(), counter, present, render);
    std::thread::spawn(move || {
        let mut app = app;
        loop {
            match app.update() {
                Ok(Status::Finished) => break,
                Err(e) => {
                    let error_message = format!("Application update error: {}", e);
                    error!(app_thread_logger.clone(), "");
                    show_error_message("Application error", error_message.as_str());
                    break;
                }
                _ => {}
            }
        }

        event_loop_proxy
            .send_event(ApplicationEvent::Finished)
            .unwrap_or_else(|_| {
                let error_message = "Can't send Finished event to event loop. It's closed already.";
                error!(app_thread_logger, "{}", error_message);
                show_error_message("Application error", error_message);
            });
    });

    event_tx
}

fn show_error_message(title: &str, message: &str) {
    message_box_ok(title, message, MessageBoxIcon::Error);
}

/// Basis structures initialization
fn base_init() -> Result<(Logger, EventLoop<ApplicationEvent>, Window), InitError> {
    let mut save_path = default_settings_path()?;
    save_path.push("HexFieldPlayground");

    // Init logger
    let logger = init_logger(save_path)?;
    info!(logger, "=============== START NEW SESSION ===============");
    trace!(logger, "Logger initilized");

    // Init event loop
    let event_loop = EventLoop::with_user_event();
    trace!(logger, "Event loop initialized");

    // Init window
    let window = init_window(&event_loop)?;
    trace!(logger, "Window initialized");

    Ok((logger, event_loop, window))
}

/// Window initialization
fn init_window(event_loop: &EventLoop<ApplicationEvent>) -> Result<Window, OsError> {
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
