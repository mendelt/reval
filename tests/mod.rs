pub mod common;

use common::{simple_event, Event};
use reval::{
    function::{FunctionContext, UserFunctions},
    prelude::*,
    value::ser::ValueSerializer,
};
use serde::Serialize;

#[tokio::test]
async fn should_ref_input() {
    // Reference the event field from the input and return it
    assert_eq!(eval_expr("event", simple_event()).await, "Simple".into())
}

#[tokio::test]
async fn should_eval_eq_on_input_event() {
    assert_eq!(
        eval_expr(r#"event == "Simple""#, simple_event()).await,
        true.into()
    )
}

#[tokio::test]
async fn should_eval_contains_expr_on_list() {
    assert_eq!(
        eval_expr(
            r#"list contains "2""#,
            Event::List {
                list: vec!["1".to_string(), "2".to_string(), "3".to_string()]
            },
        )
        .await,
        true.into(),
    );
    assert_eq!(
        eval_expr(
            r#"list contains "2""#,
            Event::List {
                list: vec!["1".to_string(), "4".to_string(), "3".to_string()]
            },
        )
        .await,
        false.into(),
    )
}

#[tokio::test]
async fn should_eval_contains_expr_on_strings() {
    assert_eq!(
        eval_expr(
            r#""this is a string" contains "is""#,
            Event::List { list: Vec::new() },
        )
        .await,
        true.into(),
    );

    assert_eq!(
        eval_expr(
            r#""this is a string" contains "something else""#,
            Event::List { list: Vec::new() },
        )
        .await,
        false.into(),
    );
}

#[tokio::test]
async fn should_eval_contains_expr_on_int() {
    assert_eq!(
        eval_expr(
            "flags contains flag",
            Event::Flags {
                flags: 0b01001000,
                flag: 0b00001000
            }
        )
        .await,
        true.into()
    );

    assert_eq!(
        eval_expr(
            "flag in flags",
            Event::Flags {
                flags: 0b01001000,
                flag: 0b00001000
            }
        )
        .await,
        true.into()
    );

    assert_eq!(
        eval_expr(
            "flag in flags",
            Event::Flags {
                flags: 0b01001000,
                flag: 0b00100000
            }
        )
        .await,
        false.into()
    );
}

/// Evaluate a simple expression against an event
async fn eval_expr<E: Serialize>(expr: &str, event: E) -> Value {
    let event = event.serialize(ValueSerializer).unwrap();

    let expr = Expr::parse(expr).unwrap();

    let functions = UserFunctions::default();
    let mut context: FunctionContext = (&functions).into();

    expr.evaluate(&mut context, &event).await.unwrap()
}
