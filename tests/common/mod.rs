//! Shared functionality, types and data for testing

use std::fs;

use chrono::prelude::*;
use reval::{prelude::*, value::ser::ValueSerializer};
use serde::Serialize;

/// Event type used for running rules on, Event can take different event types,
/// serde tag flattens the event
#[derive(Serialize)]
#[serde(tag = "event")]
pub enum Event {
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

/// Evaluate a simple expression against an event
pub async fn eval_expr<E: Serialize>(expr: &str, event: E) -> Value {
    let event = event.serialize(ValueSerializer).unwrap();

    let expr = Expr::parse(expr).unwrap();

    expr.evaluate(&event).await.unwrap()
}

pub fn parse_rule(filename: &str) -> Rule {
    Rule::parse(
        &fs::read_to_string(&format!("./tests/rules/{}.rule", filename))
            .expect("Failed to read rule file"),
    )
    .expect("Failed to parse rule")
}
