//! Evaluate Expr
//!
use crate::{error::Result, expr::Expr, function::FunctionContext, value::Value, Error};
use async_recursion::async_recursion;
use rust_decimal::prelude::*;

impl Expr {
    #[async_recursion]
    pub(crate) async fn evaluate<'a>(
        &self,
        context: &mut FunctionContext<'a>,
        facts: &Value,
    ) -> Result<Value> {
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
            Expr::Not(value) => not(value.evaluate(context, facts).await?),
            Expr::Neg(value) => neg(value.evaluate(context, facts).await?),
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
            Expr::Equals(left, right) => Ok(Value::Bool(left == right)),
            Expr::NotEquals(left, right) => Ok(Value::Bool(left != right)),
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
            Expr::And(left, right) => and(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
            Expr::Or(left, right) => or(
                left.evaluate(context, facts).await?,
                right.evaluate(context, facts).await?,
            ),
        }
    }
}

fn reference(facts: &Value, name: &str) -> Result<Value> {
    match facts {
        value if name == "facts" => Ok(value),
        Value::Map(facts) => facts
            .get(name)
            .ok_or_else(|| Error::UnknownValue(name.to_owned())),
        _ => Err(Error::InvalidType),
    }
    .map(Clone::clone)
}

fn index(value: Value, idx: Value) -> Result<Value> {
    match (&value, idx) {
        (Value::Map(map), Value::String(field)) => map
            .get(&field)
            .ok_or_else(|| Error::UnknownValue(field.to_owned())),
        (Value::Vec(vec), Value::Int(index)) => vec
            .get(index as usize)
            .ok_or_else(|| Error::UnknownValue(index.to_string())),
        (_, _) => Err(Error::InvalidType),
    }
    .map(Clone::clone)
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
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left * right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 * right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Decimal(Into::<Decimal>::into(left) * right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left * right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Decimal(left * Into::<Decimal>::into(right)))
        }

        _ => Err(Error::InvalidType),
    }
}

fn div(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left / right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 / right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Decimal(Into::<Decimal>::into(left) / right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left / right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Decimal(left / Into::<Decimal>::into(right)))
        }

        _ => Err(Error::InvalidType),
    }
}

fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left + right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 + right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Decimal(Into::<Decimal>::into(left) + right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left + right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Decimal(left + Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn sub(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left - right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 - right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Decimal(Into::<Decimal>::into(left) - right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Decimal(left - right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Decimal(left - Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn gt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left > right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Bool(left > right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Bool(left as f64 > right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left > right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Bool(Into::<Decimal>::into(left) > right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left > right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Bool(left > Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn gte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left >= right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Bool(left >= right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Bool(left as f64 >= right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left >= right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Bool(Into::<Decimal>::into(left) >= right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left >= right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Bool(left >= Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn lt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left < right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Bool(left < right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Bool((left as f64) < right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left < right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Bool(Into::<Decimal>::into(left) < right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left < right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Bool(left < Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn lte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left <= right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Bool(left <= right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Bool(left as f64 <= right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left <= right)),
        (Value::Int(left), Value::Decimal(right)) => {
            Ok(Value::Bool(Into::<Decimal>::into(left) <= right))
        }
        (Value::Decimal(left), Value::Decimal(right)) => Ok(Value::Bool(left <= right)),
        (Value::Decimal(left), Value::Int(right)) => {
            Ok(Value::Bool(left <= Into::<Decimal>::into(right)))
        }
        _ => Err(Error::InvalidType),
    }
}

fn and(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left & right)),
        _ => Err(Error::InvalidType),
    }
}

fn or(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left & right)),
        _ => Err(Error::InvalidType),
    }
}
