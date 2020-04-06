use crate::input::InputEvent::{Keyboard, Symbol};
use std::convert::TryFrom;
use std::time::Instant;
use winit::event::{
    DeviceId, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Debug, Clone)]
pub struct Input {
    happen_at: Instant,
    device_id: Option<DeviceId>,
    event: InputEvent,
}

impl From<(Option<DeviceId>, InputEvent)> for Input {
    fn from(input: (Option<DeviceId>, InputEvent)) -> Self {
        Self {
            happen_at: Instant::now(),
            device_id: input.0,
            event: input.1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Keyboard {
        key: VirtualKeyCode,
        state: ElementState,
    },
    MouseMove {
        delta: (f64, f64),
    },
    MouseButton {
        button: MouseButton,
        state: ElementState,
    },
    Symbol(char),
}

impl From<char> for InputEvent {
    fn from(c: char) -> Self {
        Symbol(c)
    }
}

impl From<(VirtualKeyCode, ElementState)> for InputEvent {
    fn from(input: (VirtualKeyCode, ElementState)) -> Self {
        Keyboard {
            key: input.0,
            state: input.1,
        }
    }
}

impl TryFrom<Event<'_, ()>> for Input {
    type Error = ();

    fn try_from(event: Event<()>) -> Result<Self, Self::Error> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ReceivedCharacter(c) => Ok((None, c.into()).into()),
                WindowEvent::KeyboardInput {
                    input, device_id, ..
                } => {
                    if let Some(key) = input.virtual_keycode {}
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}
