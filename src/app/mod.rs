pub mod error;
pub mod event;
pub mod model;

use error::UpdateError;
use event::Event;
use slog::Logger;
use std::sync::mpsc::{Receiver, RecvError, TryRecvError};

pub enum Status {
    Running,
    Finished,
}

pub struct App<Model> {
    event_rx: Receiver<Event>,
    logger: Logger,
    model: Model,
}

impl<Model: model::Model> App<Model> {
    pub fn new(event_rx: Receiver<Event>, logger: Logger, model: Model) -> Self {
        trace!(logger, "App is creating");
        Self {
            event_rx,
            logger,
            model,
        }
    }

    pub fn update(&mut self) -> Result<Status, UpdateError> {
        let events = self.fetch_events()?;
        trace!(self.logger, "{:?} events fetched.", events.len());

        trace!(self.logger, "Updating model.");
        if let model::Status::Finished = self.model.update(events.iter()) {
            return Ok(Status::Finished);
        }

        Ok(Status::Running)
    }

    fn fetch_events(&self) -> Result<Vec<Event>, RecvError> {
        const CAPACITY: usize = 128;
        let mut events = Vec::with_capacity(CAPACITY);

        loop {
            match self.event_rx.try_recv() {
                Ok(e) => events.push(e),
                Err(TryRecvError::Empty) => return Ok(events),
                Err(TryRecvError::Disconnected) => return Err(RecvError),
            }
        }
    }
}
