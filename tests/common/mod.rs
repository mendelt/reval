//! Shared functionality, types and data for testing

use chrono::prelude::*;
use reval::{expr::EvaluationContext, prelude::*, value::ser::ValueSerializer};
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
    List { list: Vec<String> },
    Flags { flags: u8, flag: u8 },
    ChronoTypes { date: DateTime<Utc>, dur: i64 },
}

impl Event {
    pub(crate) fn chrono_types() -> Self {
        Event::ChronoTypes {
            date: Utc.with_ymd_and_hms(2015, 7, 30, 3, 26, 13).unwrap(),
            dur: 3600,
        }
    }
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

    expr.evaluate(&mut EvaluationContext::default(), &event)
        .await
        .unwrap()
}

pub fn check_float(value: Value, expected: f64) {
    assert!(matches!(value, Value::Float(_)));
    if let Value::Float(value) = value {
        assert!(value - expected < 0.0000001);
    }
}
