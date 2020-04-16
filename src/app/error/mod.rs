use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::sync::mpsc::RecvError;

#[derive(Debug)]
pub enum UpdateError {
    Disconnected(&'static str),
}

impl Error for UpdateError {}

impl Display for UpdateError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            UpdateError::Disconnected(s) => write!(f, "Disconnected from parent: {}", s),
        }
    }
}

impl From<RecvError> for UpdateError {
    fn from(_: RecvError) -> Self {
        UpdateError::Disconnected("Can't receive events from parent")
    }
}
