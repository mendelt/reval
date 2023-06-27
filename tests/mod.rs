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
    let expr = r#"{"ref": "event"}"#;

    assert_eq!(eval_expr(expr, simple_event()).await, "Simple".into())
}

#[tokio::test]
async fn should_eval_eq_on_input_event() {
    let expr = r#"{"eq": [{"ref": "event"}, {"string": "Simple"}]}"#;

    assert_eq!(eval_expr(expr, simple_event()).await, true.into())
}

#[tokio::test]
async fn should_eval_contains_expr_on_list() {
    let expr = r#"{"contains": [{"ref": "list"}, {"string": "2"}]}"#;

    assert_eq!(
        eval_expr(
            expr,
            Event::List {
                list: vec!["1".to_string(), "2".to_string(), "3".to_string()]
            },
        )
        .await,
        true.into(),
    );
    assert_eq!(
        eval_expr(
            expr,
            Event::List {
                list: vec!["1".to_string(), "4".to_string(), "3".to_string()]
            },
        )
        .await,
        false.into(),
    )
}

/// Evaluate a simple expression against an event
async fn eval_expr<E: Serialize>(expr: &str, event: E) -> Value {
    let event = event.serialize(ValueSerializer).unwrap();

    let expr = Expr::parse_json(expr).unwrap();

    let functions = UserFunctions::default();
    let mut context: FunctionContext = (&functions).into();

    expr.evaluate(&mut context, &event).await.unwrap()
}
