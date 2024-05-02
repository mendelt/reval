//! Test datetime and duration functions

use crate::common::eval_expr;
use chrono::{prelude::*, TimeDelta};

#[tokio::test]
async fn should_construct_datetime_from_string() {
    assert_eq!(
        eval_expr(r#"date_time("2015-07-30T03:26:13Z")"#, ()).await,
        Utc.with_ymd_and_hms(2015, 7, 30, 3, 26, 13).unwrap().into()
    )
}

#[tokio::test]
async fn should_construct_datetime_from_timestamp() {
    assert_eq!(
        eval_expr(r#"date_time(i1438226773)"#, ()).await,
        Utc.with_ymd_and_hms(2015, 7, 30, 3, 26, 13).unwrap().into()
    )
}

#[tokio::test]
async fn should_construct_duration_from_seconds() {
    assert_eq!(
        eval_expr(r#"duration(i7200)"#, ()).await,
        TimeDelta::hours(2).into()
    )
}

#[tokio::test]
async fn should_extract_datetime_year() {
    assert_eq!(
        eval_expr(r#"year(date_time(i1438226773))"#, ()).await,
        2015.into()
    )
}

#[tokio::test]
async fn should_extract_datetime_month() {
    assert_eq!(
        eval_expr(r#"month(date_time(i1438226773))"#, ()).await,
        7.into()
    )
}

#[tokio::test]
async fn should_construct_duration_weeks() {
    assert_eq!(
        eval_expr(r#"week(i100)"#, ()).await,
        TimeDelta::weeks(100).into()
    )
}

#[tokio::test]
async fn should_extract_duration_weeks() {
    assert_eq!(eval_expr(r#"week(day(i14))"#, ()).await, 2.into())
}

#[tokio::test]
async fn should_construct_duration_days() {
    assert_eq!(
        eval_expr(r#"day(i1234)"#, ()).await,
        TimeDelta::days(1234).into()
    );
}

#[tokio::test]
async fn should_extract_datetime_day() {
    assert_eq!(
        eval_expr(r#"day(date_time(i1438226773))"#, ()).await,
        30.into()
    )
}

#[tokio::test]
async fn should_extract_duration_days() {
    assert_eq!(eval_expr(r#"day(hour(i72))"#, ()).await, 3.into())
}

#[tokio::test]
async fn should_construct_duration_hours() {
    assert_eq!(
        eval_expr(r#"hour(i72)"#, ()).await,
        TimeDelta::hours(72).into()
    )
}

#[tokio::test]
async fn should_extract_datetime_hour() {
    assert_eq!(
        eval_expr(r#"hour(date_time(i1438226773))"#, ()).await,
        3.into()
    )
}

#[tokio::test]
async fn should_extract_duration_hours() {
    assert_eq!(eval_expr(r#"hour(second(i7200))"#, ()).await, 2.into())
}

#[tokio::test]
async fn should_construct_duration_minutes() {
    assert_eq!(
        eval_expr(r#"minute(i15)"#, ()).await,
        TimeDelta::minutes(15).into()
    )
}

#[tokio::test]
async fn should_extract_datetime_minute() {
    assert_eq!(
        eval_expr(r#"minute(date_time(i1438226773))"#, ()).await,
        26.into()
    )
}

#[tokio::test]
async fn should_extract_duration_minutes() {
    assert_eq!(eval_expr(r#"minute(hour(i4))"#, ()).await, 240.into())
}

#[tokio::test]
async fn should_construct_duration_seconds() {
    assert_eq!(
        eval_expr(r#"second(i2)"#, ()).await,
        TimeDelta::seconds(2).into()
    )
}

#[tokio::test]
async fn should_extract_datetime_seconds() {
    assert_eq!(
        eval_expr(r#"second(date_time(i1438226773))"#, ()).await,
        13.into()
    )
}

#[tokio::test]
async fn should_extract_duration_seconds() {
    assert_eq!(eval_expr(r#"second(minute(i2))"#, ()).await, 120.into())
}
