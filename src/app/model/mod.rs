use crate::app::event::Event;

pub enum Status {
    Running,
    Finished,
}

pub trait Model {
    fn update<'a>(&mut self, events: impl Iterator<Item = &'a Event>) -> Status;
}
