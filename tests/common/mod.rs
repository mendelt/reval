//! Shared functionality, types and data for testing

use reval::{
    function::{FunctionContext, UserFunctions},
    prelude::*,
    value::ser::ValueSerializer,
};
use serde::Serialize;
use std::collections::HashMap;

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
    List { list: Vec<String> },
    Map { map: HashMap<String, String> },
    Flags { flags: u8, flag: u8 },
}

/// A simple event with a name and id
#[derive(Serialize, Eq, PartialEq)]
pub struct SimpleEvent {
    pub name: String,
    pub id: u32,
}

/// Evaluate a simple expression against an event
pub async fn eval_expr<E: Serialize>(expr: &str, event: E) -> Value {
    let event = event.serialize(ValueSerializer).unwrap();

    let expr = Expr::parse(expr).unwrap();

    let functions = UserFunctions::default();
    let mut context: FunctionContext = (&functions).into();

    expr.evaluate(&mut context, &event).await.unwrap()
}
