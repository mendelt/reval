//! Test reval builtin functions

use crate::common::{check_float, eval_expr};
use rust_decimal::Decimal;

#[tokio::test]
async fn should_to_lower() {
    assert_eq!(
        eval_expr(r#"to_lower("String ")"#, ()).await,
        "string ".into()
    );
}

#[tokio::test]
async fn should_to_upper() {
    assert_eq!(
        eval_expr(r#"to_upper("String ")"#, ()).await,
        "STRING ".into()
    );
}

#[tokio::test]
async fn should_trim() {
    assert_eq!(eval_expr(r#"trim("String ")"#, ()).await, "String".into());
}

#[tokio::test]
async fn should_round_float_down() {
    check_float(eval_expr(r#"round(f0.3)"#, ()).await, 0.0)
}

#[tokio::test]
async fn should_round_float_up() {
    check_float(eval_expr(r#"round(f50.5)"#, ()).await, 51.0)
}

#[tokio::test]
async fn should_round_decimal_down() {
    assert_eq!(
        eval_expr(r#"round(d0.3)"#, ()).await,
        Decimal::new(0, 0).into()
    )
}

#[tokio::test]
async fn should_round_decimal_up() {
    assert_eq!(
        eval_expr(r#"round(d50.51)"#, ()).await,
        Decimal::new(51, 0).into()
    )
}

#[tokio::test]
async fn should_floor_float() {
    check_float(eval_expr(r#"floor(f6.6)"#, ()).await, 6.0)
}

#[tokio::test]
async fn should_fract_float() {
    check_float(eval_expr(r#"fract(f20.3)"#, ()).await, 0.3)
}
