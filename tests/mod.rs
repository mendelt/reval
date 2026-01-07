mod common;
mod datetime;
mod for_map_filter;
mod iif;

use crate::common::{eval_expr, parse_rule};
use chrono::{prelude::*, TimeDelta};
use common::Event;
use reval::prelude::*;
use rstest::rstest;
use rstest_reuse::{apply, template};

#[template]
#[rstest]
#[case::builtin_lowercase("builtin_lowercase")]
#[case::builtin_uppercase("builtin_uppercase")]
#[case::builtin_trim("builtin_trim")]
#[case::builtin_contains_list_1("builtin_contains_list_1")]
#[case::builtin_contains_list_2("builtin_contains_list_2")]
#[case::builtin_round_float_down("builtin_round_float_down")]
#[case::builtin_round_float_up("builtin_round_float_up")]
#[case::builtin_round_decimal_down("builtin_round_decimal_down")]
#[case::builtin_round_decimal_up("builtin_round_decimal_up")]
#[case::builtin_floor_float("builtin_floor_float")]
#[case::builtin_fract_float("builtin_fract_float")]
#[case::ref_access_value("ref_access_value")]
#[case::symbols_simple("symbols_simple")]
#[case::symbols_complex("symbols_complex")]
fn cases(#[case] filename: &str) {}

#[apply(cases)]
#[tokio::test]

async fn should_parse_expr(#[case] filename: &str) {
    let _rule = parse_rule(filename);
}

#[apply(cases)]
#[tokio::test]
async fn should_evaluate_expr(#[case] filename: &str) {
    let rule = parse_rule(filename);

    let expected = rule
        .get_metadata("expect")
        .cloned()
        .expect("No expect metadata found");

    let input = rule.get_metadata("input").cloned().unwrap_or(Value::None);

    let ruleset = ruleset()
        .with_rule(rule)
        .expect("Error building ruleset")
        .build();

    let outcomes = ruleset
        .evaluate_value(&input)
        .await
        .expect("Failed to evaluate rule");

    assert_eq!(
        outcomes.len(),
        1,
        "Expected one outcome from rule evaluation"
    );

    let value = outcomes[0].value.as_ref().expect("Error executing rule");

    assert_eq!(value, &expected, "Expected rule to evaluate to true");
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
