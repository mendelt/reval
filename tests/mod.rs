pub mod common;

use common::simple_event;
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

/// Evaluate a simple expression against an event
async fn eval_expr<E: Serialize>(expr: &str, event: E) -> Value {
    let event = event.serialize(ValueSerializer).unwrap();

    let expr = Expr::parse_json(expr).unwrap();

    let functions = UserFunctions::default();
    let mut context: FunctionContext = (&functions).into();

    expr.evaluate(&mut context, &event).await.unwrap()
}
