pub mod event;

use event::Event;
use std::sync::mpsc::Receiver;

pub struct App {
    _event_rx: Receiver<Event>,
}

impl App {
    pub fn new(event_rx: Receiver<Event>) -> Self {
        Self {
            _event_rx: event_rx,
        }
    }
}
