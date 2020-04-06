use crate::input_controller::Input;
use slog::{error, trace, Logger};
use std::sync::mpsc::Receiver;
use crate::input_controller::InputType::Mouse;
use crate::input_controller::MouseInput::Move;

/// Logs all input from InputController
pub struct InputLogger {
    rx: Receiver<Input>,
    logger: Logger,
}

impl InputLogger {
    pub fn new(rx: Receiver<Input>, logger: Logger) -> Self {
        Self { rx, logger }
    }

    pub fn run(self) {
        loop {
            match self.rx.recv() {
                Ok(input) => {
                    if let Mouse(Move(_)) = input.input_type {
                        continue
                    }
                    println!("Got input: {:?}", input);
                    trace!(self.logger, "Got input: {:?}", input);
                },
                Err(e) => trace!(self.logger, "Can't recieve input: {}", e),
            }
        }
    }
}
