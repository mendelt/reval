//! Shared functionality, types and data for testing

use serde::Serialize;

pub fn simple_event() -> Event {
    Event::Simple(SimpleEvent {
        name: "Test name".to_owned(),
        id: 44,
    })
}

/// Event type used for running rules on, Event can take different event types,
/// serde tag flattens the event
#[derive(Serialize)]
#[serde(tag = "event")]
pub enum Event {
    Simple(SimpleEvent),
}

/// A simple event with a name and id
#[derive(Serialize, Eq, PartialEq)]
pub struct SimpleEvent {
    pub name: String,
    pub id: u32,
}
