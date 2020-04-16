pub mod error;
pub mod event;

use error::UpdateError;
use event::Event;
use slog::Logger;
use std::sync::mpsc::{Receiver, RecvError, TryRecvError};
use std::time::Duration;

pub enum Status {
    Running,
    Finished,
}

pub trait Model {
    fn update<'a>(&mut self, events: impl Iterator<Item = &'a Event>) -> Status;
}

pub trait Present {
    type Mod;
    type Rend;

    fn present(&mut self, model: &Self::Mod, render: &mut Self::Rend);
}

pub trait Render {
    fn render(&mut self);
}

pub struct App<M, P, R> {
    event_rx: Receiver<Event>,
    logger: Logger,
    model: M,
    present: P,
    render: R,
}

impl<M, P, R> App<M, P, R>
where
    M: Model,
    R: Render,
    P: Present<Rend = R, Mod = M>,
{
    pub fn new(event_rx: Receiver<Event>, logger: Logger, model: M, present: P, render: R) -> Self {
        trace!(logger, "App is creating");
        Self {
            event_rx,
            logger,
            model,
            present,
            render,
        }
    }

    pub fn update(&mut self) -> Result<Status, UpdateError> {
        let events = self.fetch_events()?;
        trace!(self.logger, "{:?} events fetched.", events.len());

        trace!(self.logger, "Updating model.");
        if let Status::Finished = self.model.update(events.iter()) {
            return Ok(Status::Finished);
        }

        self.present.present(&self.model, &mut self.render);
        self.render.render();

        std::thread::sleep(Duration::from_millis(16));

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
