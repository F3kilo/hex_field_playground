use crate::app::event::{Event, Window};
use crate::app::Model;
use crate::app::Status;
use std::time::Duration;
use winit::event::{KeyboardInput, VirtualKeyCode};

pub struct CounterModel {
    value: i32,
}

impl CounterModel {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Model for CounterModel {
    fn update<'a>(&mut self, events: impl Iterator<Item = &'a Event>) -> Status {
        for event in events {
            if let Event::Window {
                event: Window::CloseRequested,
                ..
            } = event
            {
                return Status::Finished;
            }

            if let Event::Window {
                event:
                    Window::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Up),
                                ..
                            },
                        ..
                    },
                ..
            } = event
            {
                self.value += 1;
            }

            if let Event::Window {
                event:
                    Window::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Down),
                                ..
                            },
                        ..
                    },
                ..
            } = event
            {
                self.value -= 1;
            }
        }
        
        Status::Running
    }
}
