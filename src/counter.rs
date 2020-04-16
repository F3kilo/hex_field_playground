use crate::app::event::{Event, Window};
use crate::app::model::Model;
use crate::app::model::Status;
use std::time::Duration;
use winit::event::{KeyboardInput, VirtualKeyCode};

pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    // pub fn value(&self) -> i32 {
    //     self.value
    // }
}

impl Model for Counter {
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

        std::thread::sleep(Duration::from_millis(500));

        Status::Running
    }
}
