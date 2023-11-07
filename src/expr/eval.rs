//! Evaluate Expressions

use super::{Expr, Index};
use crate::{error::Result, expr::EvaluationContext, value::Value, Error};
use async_recursion::async_recursion;
use chrono::{prelude::*, TimeDelta};
use rust_decimal::prelude::*;
use std::collections::HashMap;

impl Expr {
    #[async_recursion]
    pub async fn evaluate(&self, context: &mut EvaluationContext, facts: &Value) -> Result<Value> {
        match self {
            Expr::Value(value) => Ok(value.clone()),
            Expr::Reference(name) => reference(facts, name),
            Expr::Symbol(name) => symbol(name, context),
            Expr::Index(value, idx) => index(value.evaluate(context, facts).await?, idx),
            Expr::Function(name, value) => {
                let param = value.evaluate(context, facts).await?;
                context.call_function(name, param).await
            }
            Expr::If(switch, left, right) => iif(context, facts, switch, left, right).await,
            Expr::Not(value) => not(value.evaluate(context, facts).await?),
            Expr::Neg(value) => neg(value.evaluate(context, facts).await?),
            Expr::Some(value) => some(value.evaluate(context, facts).await?),
            Expr::None(value) => none(value.evaluate(context, facts).await?),
            Expr::DateTime(value) => datetime(value.evaluate(context, facts).await?),
            Expr::Duration(value) => duration(value.evaluate(context, facts).await?),
            Expr::Map(map) => eval_map(map, context, facts).await,
            Expr::Vec(vec) => eval_vec(vec, context, facts).await,
            Expr::Int(value) => int(value.evaluate(context, facts).await?),
            Expr::Float(value) => float(value.evaluate(context, facts).await?),
            Expr::Dec(value) => dec(value.evaluate(context, facts).await?),
            Expr::Mult(left, right) => mult(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::Div(left, right) => div(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::Add(left, right) => add(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::Sub(left, right) => sub(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::Equals(left, right) => eq(context, facts, left, right).await.map(Value::Bool),
            Expr::NotEquals(left, right) => eq(context, facts, left, right)
                .await
                .map(|val| Value::Bool(!val)),
            Expr::GreaterThan(left, right) => gt(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::GreaterThanEquals(left, right) => gte(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::LessThan(left, right) => lt(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::LessThanEquals(left, right) => lte(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),

            Expr::And(left, right) => and(context, facts, left, right).await,
            Expr::Or(left, right) => or(context, facts, left, right).await,

            Expr::BitAnd(left, right) => bitwise_and(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::BitOr(left, right) => bitwise_or(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::BitXor(left, right) => bitwise_xor(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),

            Expr::Contains(coll, item) => contains(
                coll.evaluate(context, facts).await?,
                item.evaluate(context, facts).await?,
            ),

            Expr::UpperCase(value) => uppercase(value.evaluate(context, facts).await?),
            Expr::LowerCase(value) => lowercase(value.evaluate(context, facts).await?),
            Expr::Trim(value) => trim(value.evaluate(context, facts).await?),
            Expr::Round(value) => round(value.evaluate(context, facts).await?),
            Expr::Floor(value) => floor(value.evaluate(context, facts).await?),
            Expr::Fract(value) => fract(value.evaluate(context, facts).await?),

            Expr::Year(value) => year(value.evaluate(context, facts).await?),
            Expr::Month(value) => month(value.evaluate(context, facts).await?),
            Expr::Week(value) => week(value.evaluate(context, facts).await?),
            Expr::Day(value) => day(value.evaluate(context, facts).await?),
            Expr::Hour(value) => hour(value.evaluate(context, facts).await?),
            Expr::Minute(value) => minute(value.evaluate(context, facts).await?),
            Expr::Second(value) => second(value.evaluate(context, facts).await?),
        }
    }
}

fn reference(facts: &Value, name: &str) -> Result<Value> {
    match facts {
        value if name == "facts" => Ok(value),
        Value::Map(facts) => facts
            .get(name)
            .ok_or_else(|| Error::UnknownRef(name.to_owned())),
        _ => Err(Error::InvalidType),
    }
    .cloned()
}

fn symbol(name: &str, context: &EvaluationContext<'_>) -> Result<Value> {
    context.get_symbol(name).cloned()
}

fn index(value: Value, index: &Index) -> Result<Value> {
    match (&value, index) {
        (Value::Map(map), Index::Map(field)) => Ok(map.get(field).cloned().unwrap_or(Value::None)),
        (Value::Vec(vec), Index::Vec(index)) => Ok(vec.get(*index).cloned().unwrap_or(Value::None)),
        (Value::None, _) => Ok(Value::None),
        (_, _) => Err(Error::InvalidType),
    }
}

async fn iif(
    context: &mut EvaluationContext<'_>,
    facts: &Value,
    switch: &Expr,
    left: &Expr,
    right: &Expr,
) -> Result<Value> {
    match switch.evaluate(context, facts).await? {
        Value::Bool(true) => left.evaluate(context, facts).await,
        Value::Bool(false) => right.evaluate(context, facts).await,
        _ => Err(Error::InvalidType),
    }
}

fn not(value: Value) -> Result<Value> {
    match value {
        Value::Bool(value) => Ok(Value::Bool(!value)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn neg(value: Value) -> Result<Value> {
    match value {
        Value::Int(value) => Ok(Value::Int(-value)),
        Value::Float(value) => Ok(Value::Float(-value)),
        Value::Decimal(value) => Ok(Value::Decimal(-value)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn some(value: Value) -> Result<Value> {
    match value {
        Value::None => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
    }
}

fn none(value: Value) -> Result<Value> {
    match value {
        Value::None => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

async fn eval_map(
    map: &HashMap<String, Expr>,
    context: &mut EvaluationContext<'_>,
    facts: &Value,
) -> Result<Value> {
    let mut result = HashMap::<String, Value>::new();

    for (key, expr) in map {
        result.insert(key.clone(), expr.evaluate(context, facts).await?);
    }

    Ok(result.into())
}

async fn eval_vec(
    vec: &Vec<Expr>,
    context: &mut EvaluationContext<'_>,
    facts: &Value,
) -> Result<Value> {
    let mut result = Vec::<Value>::new();
    for expr in vec {
        result.push(expr.evaluate(context, facts).await?)
    }
    Ok(result.into())
}

fn int(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(_) => Ok(value),
        Value::Float(val) => Ok((val as i128).into()),
        Value::Decimal(val) => val
            .to_i128()
            .ok_or_else(|| Error::invalid_cast(value, "Value::Int"))
            .map(Value::Int),
        Value::String(val) => i128::from_str(&val)
            .map(Value::Int)
            .map_err(|_| Error::invalid_cast(value, "Value::Int")),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn float(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(val) => Ok((val as f64).into()),
        Value::Float(_) => Ok(value),
        Value::Decimal(val) => val
            .to_f64()
            .ok_or_else(|| Error::invalid_cast(value, "Value::Float"))
            .map(Value::Float),
        Value::String(val) => f64::from_str(&val)
            .map(Value::Float)
            .map_err(|_| Error::invalid_cast(value, "Value::Float")),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn dec(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(val) => Ok(Value::Decimal(val.into())),
        Value::Float(val) => Decimal::try_from(val)
            .map(Value::Decimal)
            .map_err(|_| Error::invalid_cast(value, "Value::Float")),
        Value::Decimal(_) => Ok(value),
        Value::String(val) => Decimal::from_str(&val)
            .map(Value::Decimal)
            .map_err(|_| Error::invalid_cast(value, "Value::Decimal")),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn datetime(value: Value) -> Result<Value> {
    match value.clone() {
        Value::String(val) => val
            .parse::<DateTime<Utc>>()
            .map(Value::DateTime)
            .map_err(|_| Error::invalid_cast(value, "Value::DateTime")),
        Value::Int(val) => DateTime::from_timestamp(val as i64, 0)
            .map(Value::DateTime)
            .ok_or(Error::invalid_cast(value, "Value::DateTime")),
        Value::DateTime(_) => Ok(value),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn duration(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(val) => TimeDelta::try_seconds(val as i64)
            .map(Value::Duration)
            .ok_or(Error::invalid_cast(value, "Value::Duration")),
        Value::Duration(_) => Ok(value),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn mult(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left * right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn div(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => match left.checked_div(right) {
            Some(result) => Ok(Value::Int(result)),
            None => Err(Error::DivisionByZero),
        },
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
        (Value::Decimal(left), Value::Decimal(right)) => match left.checked_div(right) {
            Some(result) => Ok(Value::Decimal(result)),
            None => Err(Error::DivisionByZero),
        },
        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left + right)),
        (Value::DateTime(left), Value::Duration(right)) => Ok(Value::DateTime(left + right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn sub(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left - right)),
        (Value::DateTime(left), Value::DateTime(right)) => Ok(Value::Duration(left - right)),
        (Value::DateTime(left), Value::Duration(right)) => Ok(Value::DateTime(left - right)),
        (Value::Duration(left), Value::Duration(right)) => Ok(Value::Duration(left - right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

async fn eq<'a>(
    context: &mut EvaluationContext<'a>,
    facts: &Value,
    left: &Expr,
    right: &Expr,
) -> Result<bool> {
    let left = left.evaluate(context, facts).await?;

    if left == Value::None {
        // Nothing equals Value::None, not even Value::None, so early return
        return Ok(false);
    }

    let right = right.evaluate(context, facts).await?;

    Ok(left == right)
}

fn gt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left > right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left > right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left > right)),
        (Value::DateTime(left), Value::DateTime(right)) => Ok(Value::Bool(left > right)),
        (Value::Duration(left), Value::Duration(right)) => Ok(Value::Bool(left > right)),

        (Value::None, _) | (_, Value::None) => Ok(false.into()),
        _ => Err(Error::InvalidType),
    }
}

fn gte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left >= right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left >= right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left >= right)),
        (Value::DateTime(left), Value::DateTime(right)) => Ok(Value::Bool(left >= right)),
        (Value::Duration(left), Value::Duration(right)) => Ok(Value::Bool(left >= right)),

        (Value::None, _) | (_, Value::None) => Ok(false.into()),
        _ => Err(Error::InvalidType),
    }
}

fn lt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left < right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left < right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left < right)),
        (Value::DateTime(left), Value::DateTime(right)) => Ok(Value::Bool(left < right)),
        (Value::Duration(left), Value::Duration(right)) => Ok(Value::Bool(left < right)),

        (Value::None, _) | (_, Value::None) => Ok(false.into()),
        _ => Err(Error::InvalidType),
    }
}

fn lte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left <= right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left <= right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left <= right)),
        (Value::DateTime(left), Value::DateTime(right)) => Ok(Value::Bool(left <= right)),
        (Value::Duration(left), Value::Duration(right)) => Ok(Value::Bool(left <= right)),

        (Value::None, _) | (_, Value::None) => Ok(false.into()),
        _ => Err(Error::InvalidType),
    }
}

/// Lazilly evaluate an and expression
async fn and<'a>(
    context: &mut EvaluationContext<'a>,
    facts: &Value,
    left: &Expr,
    right: &Expr,
) -> Result<Value> {
    Ok(if !eval_to_bool(context, facts, left).await? {
        // If left evaluates to false bypass right and return false immediately
        false
    } else {
        // If left evaluates to true return the result of evaluating right
        eval_to_bool(context, facts, right).await?
    }
    .into())
}

/// Lazilly evaluate an or expression
async fn or<'a>(
    context: &mut EvaluationContext<'a>,
    facts: &Value,
    left: &Expr,
    right: &Expr,
) -> Result<Value> {
    Ok(if eval_to_bool(context, facts, left).await? {
        // If left evaluates to true bypass right and return true immediately
        true
    } else {
        // If left evaluates to false return the result of evaluating right
        eval_to_bool(context, facts, right).await?
    }
    .into())
}

/// Helper function that evaluates an expression and checks if its a boolean
async fn eval_to_bool<'a>(
    context: &mut EvaluationContext<'a>,
    facts: &Value,
    expr: &Expr,
) -> Result<bool> {
    TryInto::<bool>::try_into(expr.evaluate(context, facts).await?).map_err(|_| Error::InvalidType)
}

fn bitwise_and(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left & right)),
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left & right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn bitwise_or(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left | right)),
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left | right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn bitwise_xor(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left ^ right)),
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left ^ right)),

        (Value::None, _) | (_, Value::None) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn contains(coll: Value, item: Value) -> Result<Value> {
    match (coll, item) {
        (Value::Map(map), Value::String(key)) => Ok(Value::Bool(map.contains_key(&key))),
        (Value::Vec(vec), item) => Ok(Value::Bool(vec.contains(&item))),
        (Value::String(coll), Value::String(item)) => Ok(Value::Bool(coll.contains(&item))),
        (Value::Int(flags), Value::Int(flag)) => Ok(Value::Bool((flags & flag) != 0)),

        (Value::None, _) => Ok(Value::Bool(false)),
        _ => Err(Error::InvalidType),
    }
}

fn uppercase(value: Value) -> Result<Value> {
    match value {
        Value::String(value) => Ok(Value::String(value.to_uppercase())),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn lowercase(value: Value) -> Result<Value> {
    match value {
        Value::String(value) => Ok(Value::String(value.to_lowercase())),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn trim(value: Value) -> Result<Value> {
    match value {
        Value::String(inner) => Ok(Value::String(inner.trim().to_string())),
        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn floor(value: Value) -> Result<Value> {
    match value {
        Value::Float(inner) => Ok(Value::Float(inner.floor())),
        Value::Decimal(inner) => Ok(Value::Decimal(inner.floor())),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn round(value: Value) -> Result<Value> {
    match value {
        Value::Float(inner) => Ok(Value::Float(inner.round())),
        Value::Decimal(inner) => Ok(Value::Decimal(inner.round())),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn fract(value: Value) -> Result<Value> {
    match value {
        Value::Float(inner) => Ok(Value::Float(inner.fract())),
        Value::Decimal(inner) => Ok(Value::Decimal(inner.fract())),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn year(value: Value) -> Result<Value> {
    match value {
        Value::DateTime(value) => Ok(Value::Int(value.year() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn month(value: Value) -> Result<Value> {
    match value {
        Value::DateTime(inner) => Ok(Value::Int(inner.month() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn week(value: Value) -> Result<Value> {
    match &value {
        Value::Int(inner) => TimeDelta::try_weeks(*inner as i64)
            .map(Value::Duration)
            .ok_or(Error::value_out_of_bounds(value, "week")),
        Value::Duration(value) => Ok(Value::Int(value.num_weeks() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn day(value: Value) -> Result<Value> {
    match &value {
        Value::Int(inner) => TimeDelta::try_days(*inner as i64)
            .map(Value::Duration)
            .ok_or(Error::value_out_of_bounds(value, "day")),
        Value::DateTime(inner) => Ok(Value::Int(inner.day() as i128)),
        Value::Duration(inner) => Ok(Value::Int(inner.num_days() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn hour(value: Value) -> Result<Value> {
    match &value {
        Value::Int(inner) => TimeDelta::try_hours(*inner as i64)
            .map(Value::Duration)
            .ok_or(Error::value_out_of_bounds(value, "hour")),
        Value::DateTime(inner) => Ok(Value::Int(inner.hour() as i128)),
        Value::Duration(inner) => Ok(Value::Int(inner.num_hours() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn minute(value: Value) -> Result<Value> {
    match &value {
        Value::Int(inner) => TimeDelta::try_minutes(*inner as i64)
            .map(Value::Duration)
            .ok_or(Error::value_out_of_bounds(value, "minute")),
        Value::DateTime(inner) => Ok(Value::Int(inner.minute() as i128)),
        Value::Duration(inner) => Ok(Value::Int(inner.num_minutes() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn second(value: Value) -> Result<Value> {
    match &value {
        Value::Int(inner) => TimeDelta::try_seconds(*inner as i64)
            .map(Value::Duration)
            .ok_or(Error::value_out_of_bounds(value, "second")),
        Value::DateTime(inner) => Ok(Value::Int(inner.second() as i128)),
        Value::Duration(inner) => Ok(Value::Int(inner.num_seconds() as i128)),

        Value::None => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}
