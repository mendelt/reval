//! Evaluate Expressions

mod context;

use crate::{
    error::{Error, Result},
    expr::{Expr, Index},
    function::FunctionCache,
    ruleset::RuleSet,
    value::Value,
};
use async_recursion::async_recursion;
use chrono::{prelude::*, TimeDelta};
use context::EvalContext;
use rust_decimal::prelude::*;
use std::collections::BTreeMap;

impl Expr {
    /// Evaluate the Expr, passing in a set of values
    pub async fn evaluate(&self, facts: &Value) -> Result<Value> {
        self.eval_rec(&mut EvalContext::new(
            &EMPTY_RULES,
            &mut FunctionCache::new(),
            facts,
        ))
        .await
    }

    /// Evaluate the expression in the context of a rule
    pub(crate) async fn eval_rule(
        &self,
        ruleset: &RuleSet,
        function_cache: &mut FunctionCache,
        facts: &Value,
    ) -> Result<Value> {
        self.eval_rec(&mut EvalContext::new(ruleset, function_cache, facts))
            .await
    }

    /// Recursively evaluate an expression
    #[async_recursion]
    async fn eval_rec(&self, context: &mut EvalContext) -> Result<Value> {
        match self {
            Expr::Value(value) => Ok(value.clone()),
            Expr::Reference(name) => context.reference(name),
            Expr::Symbol(name) => context.symbol(name),
            Expr::Index(value, idx) => index(value.eval_rec(context).await?, idx),
            Expr::Function(name, value) => {
                let param = value.eval_rec(context).await?;
                context.call_function(name, param).await
            }
            Expr::If(switch, left, right) => iif(context, switch, left, right).await,
            Expr::Not(value) => not(value.eval_rec(context).await?),
            Expr::Neg(value) => neg(value.eval_rec(context).await?),
            Expr::Some(value) => some(value.eval_rec(context).await?),
            Expr::None(value) => none(value.eval_rec(context).await?),
            Expr::DateTime(value) => datetime(value.eval_rec(context).await?),
            Expr::Duration(value) => duration(value.eval_rec(context).await?),
            Expr::Map(map) => eval_map(map, context).await,
            Expr::Vec(vec) => eval_vec(vec, context).await,
            Expr::Int(value) => int(value.eval_rec(context).await?),
            Expr::Float(value) => float(value.eval_rec(context).await?),
            Expr::Dec(value) => dec(value.eval_rec(context).await?),
            Expr::Mult(left, right) => mult(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Div(left, right) => div(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Rem(left, right) => rem(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Add(left, right) => add(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Sub(left, right) => sub(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Equals(left, right) => eq(context, left, right).await.map(Value::Bool),
            Expr::NotEquals(left, right) => {
                eq(context, left, right).await.map(|val| Value::Bool(!val))
            }
            Expr::GreaterThan(left, right) => gt(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::GreaterThanEquals(left, right) => gte(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::LessThan(left, right) => lt(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::LessThanEquals(left, right) => lte(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::And(left, right) => and(context, left, right).await,
            Expr::Or(left, right) => or(context, left, right).await,
            Expr::BitAnd(left, right) => bitwise_and(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::BitOr(left, right) => bitwise_or(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::BitXor(left, right) => bitwise_xor(
                left.eval_rec(context).await?,
                right.eval_rec(context).await?,
            ),
            Expr::Contains(coll, item) => {
                contains(coll.eval_rec(context).await?, item.eval_rec(context).await?)
            }
            Expr::Starts(string, substr) => starts(
                string.eval_rec(context).await?,
                substr.eval_rec(context).await?,
            ),
            Expr::Ends(string, substr) => ends(
                string.eval_rec(context).await?,
                substr.eval_rec(context).await?,
            ),
            Expr::UpperCase(value) => uppercase(value.eval_rec(context).await?),
            Expr::LowerCase(value) => lowercase(value.eval_rec(context).await?),
            Expr::Trim(value) => trim(value.eval_rec(context).await?),
            Expr::Round(value) => round(value.eval_rec(context).await?),
            Expr::Floor(value) => floor(value.eval_rec(context).await?),
            Expr::Fract(value) => fract(value.eval_rec(context).await?),
            Expr::Year(value) => year(value.eval_rec(context).await?),
            Expr::Month(value) => month(value.eval_rec(context).await?),
            Expr::Week(value) => week(value.eval_rec(context).await?),
            Expr::Day(value) => day(value.eval_rec(context).await?),
            Expr::Hour(value) => hour(value.eval_rec(context).await?),
            Expr::Minute(value) => minute(value.eval_rec(context).await?),
            Expr::Second(value) => second(value.eval_rec(context).await?),
        }
    }
}

lazy_static::lazy_static!(
    static ref EMPTY_RULES: RuleSet = {
        Default::default()
    };
);

fn index(value: Value, index: &Index) -> Result<Value> {
    match (&value, index) {
        (Value::Map(map), Index::Map(field)) => Ok(map.get(field).cloned().unwrap_or(Value::None)),
        (Value::Vec(vec), Index::Vec(index)) => Ok(vec.get(*index).cloned().unwrap_or(Value::None)),
        (Value::None, _) => Ok(Value::None),
        (_, _) => Err(Error::InvalidType),
    }
}

async fn iif(
    context: &mut EvalContext<'_>,
    switch: &Expr,
    left: &Expr,
    right: &Expr,
) -> Result<Value> {
    match switch.eval_rec(context).await? {
        Value::Bool(true) => left.eval_rec(context).await,
        Value::Bool(false) => right.eval_rec(context).await,
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

async fn eval_map(map: &BTreeMap<String, Expr>, context: &mut EvalContext<'_>) -> Result<Value> {
    let mut result = BTreeMap::<String, Value>::new();

    for (key, expr) in map {
        result.insert(key.clone(), expr.eval_rec(context).await?);
    }

    Ok(result.into())
}

async fn eval_vec(vec: &Vec<Expr>, context: &mut EvalContext<'_>) -> Result<Value> {
    let mut result = Vec::<Value>::new();
    for expr in vec {
        result.push(expr.eval_rec(context).await?)
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

fn rem(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => match left.checked_rem(right) {
            Some(result) => Ok(Value::Int(result)),
            None => Err(Error::DivisionByZero),
        },
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left % right)),
        (Value::Decimal(left), Value::Decimal(right)) => match left.checked_rem(right) {
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

async fn eq(context: &mut EvalContext<'_>, left: &Expr, right: &Expr) -> Result<bool> {
    let left = left.eval_rec(context).await?;

    if left == Value::None {
        // Nothing equals Value::None, not even Value::None, so early return
        return Ok(false);
    }

    let right = right.eval_rec(context).await?;

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
async fn and(context: &mut EvalContext<'_>, left: &Expr, right: &Expr) -> Result<Value> {
    Ok(if !eval_to_bool(context, left).await? {
        // If left evaluates to false bypass right and return false immediately
        false
    } else {
        // If left evaluates to true return the result of evaluating right
        eval_to_bool(context, right).await?
    }
    .into())
}

/// Lazilly evaluate an or expression
async fn or(context: &mut EvalContext<'_>, left: &Expr, right: &Expr) -> Result<Value> {
    Ok(if eval_to_bool(context, left).await? {
        // If left evaluates to true bypass right and return true immediately
        true
    } else {
        // If left evaluates to false return the result of evaluating right
        eval_to_bool(context, right).await?
    }
    .into())
}

/// Helper function that evaluates an expression and checks if its a boolean
async fn eval_to_bool(context: &mut EvalContext<'_>, expr: &Expr) -> Result<bool> {
    TryInto::<bool>::try_into(expr.eval_rec(context).await?).map_err(|_| Error::InvalidType)
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

fn starts(string: Value, substr: Value) -> Result<Value> {
    match (string, substr) {
        (Value::String(string), Value::String(prefix)) => {
            Ok(Value::Bool(string.starts_with(&prefix)))
        }

        (Value::None, _) => Ok(Value::None),
        _ => Err(Error::InvalidType),
    }
}

fn ends(string: Value, substr: Value) -> Result<Value> {
    match (string, substr) {
        (Value::String(string), Value::String(suffix)) => {
            Ok(Value::Bool(string.ends_with(&suffix)))
        }

        (Value::None, _) => Ok(Value::None),
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
