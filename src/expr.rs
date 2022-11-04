use displaydoc::Display;
use thiserror::Error;

use crate::value::Value;

#[derive(Debug, Error, Display)]
pub enum Error {
    /// An invalid type was encountered evaluating the expression
    InvalidValueType,

    /// Missing value
    MissingValue(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value
    Value(Value),

    /// A reference to a value
    Reference(String),

    /// Indexes a dictionary or an array value
    Index(Box<Expr>, Box<Expr>),

    /// Negates a subexpression !true evaluates to false
    Not(Box<Expr>),

    /// Multiply two subexpressions
    Mult(Box<Expr>, Box<Expr>),

    /// Divide two subexpressions
    Div(Box<Expr>, Box<Expr>),

    /// Add two subexpressions
    Add(Box<Expr>, Box<Expr>),

    /// Subtract two subexpressions
    Sub(Box<Expr>, Box<Expr>),

    /// Equates two subexpressions
    Equals(Box<Expr>, Box<Expr>),

    /// Inverse equation of two subexpressions
    NotEquals(Box<Expr>, Box<Expr>),

    /// Checks if one of two subexpressions is greater than the other
    GreaterThan(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpressions is greater than or equal to the other
    GreaterThanEquals(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpression is less than the other
    LessThan(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpression is less than or equal to the other
    LessThanEquals(Box<Expr>, Box<Expr>),

    /// And operation on two subexpressions
    And(Box<Expr>, Box<Expr>),

    /// Or operation on two subexpressions
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(self, facts: &Value) -> Result<Value, Error> {
        match self {
            Expr::Value(value) => Ok(value),
            Expr::Reference(name) => reference(facts, name), // Ok(facts.get(&name).clone()),
            Expr::Index(value, idx) => index(value.evaluate(facts)?, idx.evaluate(facts)?),
            Expr::Not(value) => not(value.evaluate(facts)?),
            Expr::Mult(left, right) => mult(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::Div(left, right) => div(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::Add(left, right) => add(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::Sub(left, right) => sub(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::Equals(left, right) => Ok(Value::Bool(left == right)),
            Expr::NotEquals(left, right) => Ok(Value::Bool(left != right)),
            Expr::GreaterThan(left, right) => gt(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::GreaterThanEquals(left, right) => {
                gte(left.evaluate(facts)?, right.evaluate(facts)?)
            }
            Expr::LessThan(left, right) => lt(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::LessThanEquals(left, right) => lte(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::And(left, right) => and(left.evaluate(facts)?, right.evaluate(facts)?),
            Expr::Or(left, right) => or(left.evaluate(facts)?, right.evaluate(facts)?),
        }
    }

    pub fn value(value: impl Into<Value>) -> Self {
        Expr::Value(value.into())
    }

    pub fn index(left: Expr, right: Expr) -> Self {
        Expr::Index(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Expr) -> Self {
        Expr::Not(Box::new(expr))
    }

    pub fn mult(left: Expr, right: Expr) -> Self {
        Expr::Mult(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Div(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Sub(Box::new(left), Box::new(right))
    }

    pub fn eq(left: Expr, right: Expr) -> Self {
        Expr::Equals(Box::new(left), Box::new(right))
    }

    pub fn neq(left: Expr, right: Expr) -> Self {
        Expr::NotEquals(Box::new(left), Box::new(right))
    }

    pub fn gt(left: Expr, right: Expr) -> Self {
        Expr::GreaterThan(Box::new(left), Box::new(right))
    }

    pub fn gte(left: Expr, right: Expr) -> Self {
        Expr::GreaterThanEquals(Box::new(left), Box::new(right))
    }

    pub fn lt(left: Expr, right: Expr) -> Self {
        Expr::LessThan(Box::new(left), Box::new(right))
    }

    pub fn lte(left: Expr, right: Expr) -> Self {
        Expr::LessThanEquals(Box::new(left), Box::new(right))
    }

    pub fn and(left: Expr, right: Expr) -> Self {
        Expr::And(Box::new(left), Box::new(right))
    }

    pub fn or(left: Expr, right: Expr) -> Self {
        Expr::Or(Box::new(left), Box::new(right))
    }
}

fn reference(facts: &Value, name: String) -> Result<Value, Error> {
    match facts {
        value if &name == "facts" => Ok(value),
        Value::Map(facts) => facts
            .get(&name)
            .ok_or_else(|| Error::MissingValue(name.to_owned())),
        _ => Err(Error::InvalidValueType),
    }
    .map(Clone::clone)
}

fn index(value: Value, idx: Value) -> Result<Value, Error> {
    match (&value, idx) {
        (Value::Map(map), Value::String(field)) => map
            .get(&field)
            .ok_or_else(|| Error::MissingValue(field.to_owned())),
        (Value::Vec(vec), Value::Int(index)) => vec
            .get(index as usize)
            .ok_or_else(|| Error::MissingValue(index.to_string())),
        (_, _) => Err(Error::InvalidValueType),
    }
    .map(Clone::clone)
}

fn not(value: Value) -> Result<Value, Error> {
    match value {
        Value::Bool(value) => Ok(Value::Bool(!value)),
        _ => Err(Error::InvalidValueType),
    }
}

fn mult(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left * right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 * right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),

        _ => Err(Error::InvalidValueType),
    }
}

fn div(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left / right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 / right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),

        _ => Err(Error::InvalidValueType),
    }
}

fn add(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left + right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 + right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),

        _ => Err(Error::InvalidValueType),
    }
}

fn sub(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
        (Value::Float(left), Value::Int(right)) => Ok(Value::Float(left - right as f64)),
        (Value::Int(left), Value::Float(right)) => Ok(Value::Float(left as f64 - right)),
        (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),

        _ => Err(Error::InvalidValueType),
    }
}

fn gt(_left: Value, _right: Value) -> Result<Value, Error> {
    todo!()
}

fn gte(_left: Value, _right: Value) -> Result<Value, Error> {
    todo!()
}

fn lt(_left: Value, _right: Value) -> Result<Value, Error> {
    todo!()
}

fn lte(_left: Value, _right: Value) -> Result<Value, Error> {
    todo!()
}

fn and(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left & right)),
        _ => Err(Error::InvalidValueType),
    }
}

fn or(left: Value, right: Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left & right)),
        _ => Err(Error::InvalidValueType),
    }
}
