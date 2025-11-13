mod builtin;
mod common;
mod datetime;
mod iif;
mod symbols;

use crate::common::eval_expr;
use chrono::{prelude::*, TimeDelta};
use common::{simple_event, Event};

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

#[tokio::test]
async fn should_eval_starts() {
    assert_eq!(
        eval_expr(r#""hello world" starts "hello""#, ()).await,
        true.into()
    );
    assert_eq!(
        eval_expr(r#""hello world" starts "world""#, ()).await,
        false.into()
    );
}

#[tokio::test]
async fn should_eval_ends() {
    assert_eq!(
        eval_expr(r#""hello world" ends "world""#, ()).await,
        true.into()
    );
    assert_eq!(
        eval_expr(r#""hello world" ends "hello""#, ()).await,
        false.into()
    );
}

#[tokio::test]
async fn should_eval_bitwise_and() {
    assert_eq!(eval_expr("0b010111 & 0b011101", ()).await, 0b010101.into());
    assert_eq!(eval_expr("true & false", ()).await, false.into());
}

#[tokio::test]
async fn should_eval_bitwise_or() {
    assert_eq!(eval_expr("0b010111 | 0b011101", ()).await, 0b011111.into());
    assert_eq!(eval_expr("true | false", ()).await, true.into());
}

#[tokio::test]
async fn should_eval_bitwise_xor() {
    assert_eq!(eval_expr("0b010111 ^ 0b011101", ()).await, 0b001010.into());
    assert_eq!(eval_expr("true ^ false", ()).await, true.into());
}

#[tokio::test]
async fn should_pass_in_chrono_datetime() {
    assert_eq!(
        eval_expr("date_time(date)", Event::chrono_types()).await,
        Utc.with_ymd_and_hms(2015, 7, 30, 3, 26, 13).unwrap().into()
    )
}

#[tokio::test]
async fn should_pass_in_chrono_timedelta() {
    assert_eq!(
        eval_expr("duration(dur)", Event::chrono_types()).await,
        TimeDelta::seconds(3600).into()
    )
}
