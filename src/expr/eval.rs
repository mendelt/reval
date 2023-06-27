//! Evaluate Expressions

use crate::{error::Result, expr::Expr, function::FunctionContext, value::Value, Error};
use async_recursion::async_recursion;
use rust_decimal::prelude::*;
use std::collections::HashMap;

impl Expr {
    #[async_recursion]
    pub async fn evaluate(&self, context: &mut FunctionContext, facts: &Value) -> Result<Value> {
        match self {
            Expr::Value(value) => Ok(value.clone()),
            Expr::Reference(name) => reference(facts, name),
            Expr::Index(value, idx) => index(
                value.evaluate(context, facts).await?,
                idx.evaluate(context, facts).await?,
            ),
            Expr::Function(name, value) => {
                let param = value.evaluate(context, facts).await?;
                context.call(name, param).await
            }
            Expr::If(switch, left, right) => iif(context, facts, switch, left, right).await,
            Expr::Not(value) => not(value.evaluate(context, facts).await?),
            Expr::Neg(value) => neg(value.evaluate(context, facts).await?),
            Expr::IsSome(value) => is_some(value.evaluate(context, facts).await?),
            Expr::IsNone(value) => is_none(value.evaluate(context, facts).await?),
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
    .map(Clone::clone)
}

fn index(value: Value, idx: Value) -> Result<Value> {
    match (&value, idx) {
        (Value::Map(map), Value::String(field)) => map
            .get(&field)
            .ok_or_else(|| Error::UnknownIndex(field.to_owned())),
        (Value::Vec(vec), Value::Int(index)) => vec
            .get(index as usize)
            .ok_or_else(|| Error::UnknownIndex(index.to_string())),
        (_, _) => Err(Error::InvalidType),
    }
    .map(Clone::clone)
}

async fn iif(
    context: &mut FunctionContext<'_>,
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
        _ => Err(Error::InvalidType),
    }
}

fn neg(value: Value) -> Result<Value> {
    match value {
        Value::Int(value) => Ok(Value::Int(-value)),
        Value::Float(value) => Ok(Value::Float(-value)),
        Value::Decimal(value) => Ok(Value::Decimal(-value)),

        _ => Err(Error::InvalidType),
    }
}

fn is_some(value: Value) -> Result<Value> {
    match value {
        Value::None => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
    }
}

fn is_none(value: Value) -> Result<Value> {
    match value {
        Value::None => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

async fn eval_map(
    map: &HashMap<String, Expr>,
    context: &mut FunctionContext<'_>,
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
    context: &mut FunctionContext<'_>,
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
            .ok_or_else(|| Error::InvalidCast(value, "Value::Int".to_owned()))
            .map(Value::Int),
        Value::String(val) => i128::from_str(&val)
            .map(Value::Int)
            .map_err(|_| Error::InvalidCast(value, "Value::Int".to_owned())),

        _ => Err(Error::InvalidType),
    }
}

fn float(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(val) => Ok((val as f64).into()),
        Value::Float(_) => Ok(value),
        Value::Decimal(val) => val
            .to_f64()
            .ok_or_else(|| Error::InvalidCast(value, "Value::Float".to_owned()))
            .map(Value::Float),
        Value::String(val) => f64::from_str(&val)
            .map(Value::Float)
            .map_err(|_| Error::InvalidCast(value, "Value::Float".to_owned())),

        _ => Err(Error::InvalidType),
    }
}

fn dec(value: Value) -> Result<Value> {
    match value.clone() {
        Value::Int(val) => Ok(Value::Decimal(val.into())),
        Value::Float(val) => Decimal::try_from(val)
            .map(Value::Decimal)
            .map_err(|_| Error::InvalidCast(value, "Value::Float".to_owned())),
        Value::Decimal(_) => Ok(value),
        Value::String(val) => Decimal::from_str(&val)
            .map(Value::Decimal)
            .map_err(|_| Error::InvalidCast(value, "Value::Decimal".to_owned())),

        _ => Err(Error::InvalidType),
    }
}

fn mult(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left * right)),

        _ => Err(Error::InvalidType),
    }
}

fn div(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left / right)),

        _ => Err(Error::InvalidType),
    }
}

fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left + right)),

        _ => Err(Error::InvalidType),
    }
}

fn sub(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left - right)),

        _ => Err(Error::InvalidType),
    }
}

async fn eq<'a>(
    context: &mut FunctionContext<'a>,
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

        _ => Err(Error::InvalidType),
    }
}

fn gte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left >= right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left >= right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left >= right)),

        _ => Err(Error::InvalidType),
    }
}

fn lt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left < right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left < right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left < right)),

        _ => Err(Error::InvalidType),
    }
}

fn lte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left <= right)),
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left <= right)),
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left <= right)),

        _ => Err(Error::InvalidType),
    }
}

/// Lazilly evaluate an and expression
async fn and<'a>(
    context: &mut FunctionContext<'a>,
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
    context: &mut FunctionContext<'a>,
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
    context: &mut FunctionContext<'a>,
    facts: &Value,
    expr: &Expr,
) -> Result<bool> {
    TryInto::<bool>::try_into(expr.evaluate(context, facts).await?).map_err(|_| Error::InvalidType)
}
