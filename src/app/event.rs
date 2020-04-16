use std::convert::TryFrom;
use std::path::PathBuf;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    AxisId, DeviceEvent, DeviceId, ElementState, KeyboardInput, ModifiersState, MouseButton,
    MouseScrollDelta, Touch, TouchPhase,
};
use winit::window::{Theme, WindowId};

#[derive(Debug)]
pub enum ApplicationEvent {
    Finished,
}

#[derive(Debug)]
pub enum Window {
    Resized(PhysicalSize<u32>),
    Moved(PhysicalPosition<i32>),
    CloseRequested,
    Destroyed,
    DroppedFile(PathBuf),
    HoveredFile(PathBuf),
    HoveredFileCancelled,
    ReceivedCharacter(char),
    Focused(bool),
    KeyboardInput {
        device_id: DeviceId,
        input: KeyboardInput,
        is_synthetic: bool,
    },
    ModifiersChanged(ModifiersState),
    CursorMoved {
        device_id: DeviceId,
        position: PhysicalPosition<f64>,
    },
    CursorEntered {
        device_id: DeviceId,
    },
    CursorLeft {
        device_id: DeviceId,
    },
    MouseWheel {
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
    MouseInput {
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    },
    TouchpadPressure {
        device_id: DeviceId,
        pressure: f32,
        stage: i64,
    },
    AxisMotion {
        device_id: DeviceId,
        axis: AxisId,
        value: f64,
    },
    Touch(Touch),
    ScaleFactorChanged {
        scale_factor: f64,
        new_inner_size: PhysicalSize<u32>,
    },
    ThemeChanged(Theme),
}

impl<'a> From<winit::event::WindowEvent<'a>> for Window {
    fn from(e: winit::event::WindowEvent<'a>) -> Self {
        match e {
            winit::event::WindowEvent::Resized(size) => Window::Resized(size),
            winit::event::WindowEvent::Moved(pos) => Window::Moved(pos),
            winit::event::WindowEvent::CloseRequested => Window::CloseRequested,
            winit::event::WindowEvent::Destroyed => Window::Destroyed,
            winit::event::WindowEvent::DroppedFile(path) => Window::DroppedFile(path),
            winit::event::WindowEvent::HoveredFile(path) => Window::HoveredFile(path),
            winit::event::WindowEvent::HoveredFileCancelled => Window::HoveredFileCancelled,
            winit::event::WindowEvent::ReceivedCharacter(c) => Window::ReceivedCharacter(c),
            winit::event::WindowEvent::Focused(f) => Window::Focused(f),
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => Window::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            },
            winit::event::WindowEvent::ModifiersChanged(state) => Window::ModifiersChanged(state),
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => Window::CursorMoved {
                device_id,
                position,
            },
            winit::event::WindowEvent::CursorEntered { device_id } => {
                Window::CursorEntered { device_id }
            }
            winit::event::WindowEvent::CursorLeft { device_id } => Window::CursorLeft { device_id },
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => Window::MouseWheel {
                device_id,
                delta,
                phase,
            },
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => Window::MouseInput {
                device_id,
                state,
                button,
            },
            winit::event::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => Window::TouchpadPressure {
                device_id,
                pressure,
                stage,
            },
            winit::event::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => Window::AxisMotion {
                device_id,
                axis,
                value,
            },
            winit::event::WindowEvent::Touch(touch) => Window::Touch(touch),
            winit::event::WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => Window::ScaleFactorChanged {
                new_inner_size: *new_inner_size,
                scale_factor,
            },
            winit::event::WindowEvent::ThemeChanged(theme) => Window::ThemeChanged(theme),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Window {
        window_id: WindowId,
        event: Window,
    },
    Device {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    Suspended,
    Resumed,
}

impl<'a> TryFrom<winit::event::Event<'a, ApplicationEvent>> for Event {
    type Error = winit::event::Event<'a, ApplicationEvent>;

    fn try_from(event: winit::event::Event<'a, ApplicationEvent>) -> Result<Self, Self::Error> {
        match event {
            winit::event::Event::WindowEvent { window_id, event } => Ok(Event::Window {
                window_id,
                event: event.into(),
            }),
            winit::event::Event::DeviceEvent { device_id, event } => {
                Ok(Event::Device { device_id, event })
            }
            winit::event::Event::Suspended => Ok(Event::Suspended),
            winit::event::Event::Resumed => Ok(Event::Resumed),
            _ => Err(event),
        }
    }
}
