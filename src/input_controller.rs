use slog::{error, Logger};
use std::sync::mpsc::Sender;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseScrollDelta, VirtualKeyCode};

#[derive(Debug, Clone)]
pub struct InputController {
    sender: Sender<Input>,
    logger: Logger,
}

impl InputController {
    pub fn new(sender: Sender<Input>, logger: Logger) -> Self {
        Self { sender, logger }
    }

    pub fn process_input(self, event: DeviceEvent, device_id: DeviceId) {
        match event {
            DeviceEvent::Added => self.send_or_log(Input {
                device_id,
                input_type: InputType::Added,
            }),
            DeviceEvent::Removed => self.send_or_log(Input {
                device_id,
                input_type: InputType::Removed,
            }),
            DeviceEvent::MouseMotion { delta } => self.send_or_log(Input {
                device_id,
                input_type: InputType::Mouse(MouseInput::Move(delta)),
            }),
            DeviceEvent::MouseWheel { delta } => self.send_or_log(Input {
                device_id,
                input_type: InputType::Mouse(MouseInput::Scroll(delta.into())),
            }),
            DeviceEvent::Button { button, state } => self.send_or_log(Input {
                device_id,
                input_type: InputType::Mouse(MouseInput::Button(
                    self.get_mouse_button_input(button, state),
                )),
            }),
            _ => {}
        }
    }

    fn get_mouse_button_input(&self, button_id: u32, state: ElementState) -> ButtonInput {
        ButtonInput {
            button: button_id.into(),
            state: match state {
                ElementState::Pressed => ButtonState::Pressed,
                ElementState::Released => ButtonState::Released,
            },
        }
    }

    /// Sends event with sender or logs error
    fn send_or_log(&self, input: Input) {
        self.sender
            .send(input)
            .unwrap_or_else(|e| error!(self.logger, "can't send input: {}", e))
    }
}

#[derive(Debug, Clone)]
pub enum InputType {
    Added,
    Removed,
    Mouse(MouseInput),
    Keyboard(KeyboardInput),
}

#[derive(Debug, Clone)]
pub struct Scroll {
    horizontal: f32,
    vertical: f32,
}

impl From<MouseScrollDelta> for Scroll {
    fn from(delta: MouseScrollDelta) -> Self {
        let (horizontal, vertical) = match delta {
            MouseScrollDelta::LineDelta(h, v) => (h, v),
            MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
        };
        Scroll {
            horizontal,
            vertical,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonInput {
    button: MouseButton,
    state: ButtonState,
}

#[derive(Debug, Clone)]
pub enum MouseInput {
    Move((f64, f64)),
    Button(ButtonInput),
    Scroll(Scroll),
}

#[derive(Debug, Clone)]
pub enum ButtonState {
    Pressed,
    Released,
    Click,
    DblClick,
}

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Wheel,
    Additional(u32),
}

impl From<u32> for MouseButton {
    fn from(button_id: u32) -> Self {
        MouseButton::Additional(button_id)
    }
}

#[derive(Debug, Clone)]
pub enum KeyboardInput {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub device_id: DeviceId,
    pub input_type: InputType,
}
