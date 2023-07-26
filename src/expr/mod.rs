mod eval;
pub(crate) mod keywords;

use crate::value::Value;
use std::{collections::HashMap, fmt::Display};

/// The Reval expression abstract syntax tree
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value
    Value(Value),

    /// Access a parameter passed in to the expression
    Reference(String),

    /// Evaluate a user functions by name
    Function(String, Box<Expr>),

    /// Index a dictionary or an array value
    Index(Box<Expr>, Box<Expr>),

    /// Evaluates to one of two expressions based on the boolean value of the first expression
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Construct a map from expression results
    Map(HashMap<String, Expr>),

    /// Construct a vector from expression results
    Vec(Vec<Expr>),

    /// Invert a boolean subexpression !true evaluates to false
    Not(Box<Expr>),

    /// Invert the sign of a numerical value
    Neg(Box<Expr>),

    /// True if the expression is not None
    IsSome(Box<Expr>),

    /// True if the expression is none
    IsNone(Box<Expr>),

    /// Cast numerical values to int
    Int(Box<Expr>),

    /// Cast numerical values to float
    Float(Box<Expr>),

    /// Cast numerical values to decimal
    Dec(Box<Expr>),

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

    /// Checks if a vec contains an item or if a map contains a key
    Contains(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Value expression constructor
    pub fn value(value: impl Into<Value>) -> Self {
        Expr::Value(value.into())
    }

    /// None value expression constructor
    pub fn none() -> Self {
        Expr::Value(Value::None)
    }

    /// Function expression constructor
    pub fn func(name: impl Into<String>, param: Expr) -> Self {
        Expr::Function(name.into(), Box::new(param))
    }

    /// Reference an input value
    pub fn reff(name: impl ToString) -> Self {
        Expr::Reference(name.to_string())
    }

    /// Index expression constructor
    pub fn index(value: Expr, index: Expr) -> Self {
        Expr::Index(Box::new(value), Box::new(index))
    }

    /// If expression constructor
    pub fn iif(swith: impl Into<Expr>, yes: Expr, no: Expr) -> Self {
        Expr::If(Box::new(swith.into()), Box::new(yes), Box::new(no))
    }

    /// Not expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Expr) -> Self {
        Expr::Not(Box::new(expr))
    }

    /// Neg expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn neg(expr: Expr) -> Self {
        Expr::Neg(Box::new(expr))
    }

    /// IsSome epression constructor
    pub fn is_some(expr: Expr) -> Self {
        Expr::IsSome(Box::new(expr))
    }

    /// IsNone epression constructor
    pub fn is_none(expr: Expr) -> Self {
        Expr::IsNone(Box::new(expr))
    }

    /// Int-cast expression constructor
    pub fn int(expr: Expr) -> Self {
        Expr::Int(Box::new(expr))
    }

    /// Float-cast expression constructor
    pub fn float(expr: Expr) -> Self {
        Expr::Float(Box::new(expr))
    }

    /// Decimal-cast expression constructor
    pub fn dec(expr: Expr) -> Self {
        Expr::Dec(Box::new(expr))
    }

    /// Multiply-expression constructor
    pub fn mult(left: Expr, right: Expr) -> Self {
        Expr::Mult(Box::new(left), Box::new(right))
    }

    /// Divide-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Div(Box::new(left), Box::new(right))
    }

    /// Add-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    /// Subtract-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Sub(Box::new(left), Box::new(right))
    }

    /// Equals-expression constructor
    pub fn eq(left: Expr, right: Expr) -> Self {
        Expr::Equals(Box::new(left), Box::new(right))
    }

    /// Not-equals expression constructor
    pub fn neq(left: Expr, right: Expr) -> Self {
        Expr::NotEquals(Box::new(left), Box::new(right))
    }

    /// Greater-than expression constructor
    pub fn gt(left: Expr, right: Expr) -> Self {
        Expr::GreaterThan(Box::new(left), Box::new(right))
    }

    /// Greater-than-or-equals expression constructor
    pub fn gte(left: Expr, right: Expr) -> Self {
        Expr::GreaterThanEquals(Box::new(left), Box::new(right))
    }

    /// Less-than expression constructor
    pub fn lt(left: Expr, right: Expr) -> Self {
        Expr::LessThan(Box::new(left), Box::new(right))
    }

    /// Less-than-or-equals expression constructor
    pub fn lte(left: Expr, right: Expr) -> Self {
        Expr::LessThanEquals(Box::new(left), Box::new(right))
    }

    /// And expression constructor
    pub fn and(left: Expr, right: Expr) -> Self {
        Expr::And(Box::new(left), Box::new(right))
    }

    /// Or expression constructor
    pub fn or(left: Expr, right: Expr) -> Self {
        Expr::Or(Box::new(left), Box::new(right))
    }

    pub fn contains(list: Expr, key: Expr) -> Self {
        Expr::Contains(Box::new(list), Box::new(key))
    }
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}

impl Display for Expr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Value(value) => write!(formatter, "{value}"),
            Expr::Reference(_) => todo!(),
            Expr::Function(_, _) => todo!(),
            Expr::Index(_, _) => todo!(),
            Expr::If(_, _, _) => todo!(),
            Expr::Map(_) => todo!(),
            Expr::Vec(_) => todo!(),
            Expr::Not(_) => todo!(),
            Expr::Neg(_) => todo!(),
            Expr::IsSome(_) => todo!(),
            Expr::IsNone(_) => todo!(),
            Expr::Int(_) => todo!(),
            Expr::Float(_) => todo!(),
            Expr::Dec(_) => todo!(),
            Expr::Mult(left, right) => write!(formatter, "({left} * {right})"),
            Expr::Div(left, right) => write!(formatter, "({left} / {right})"),
            Expr::Add(left, right) => write!(formatter, "({left} + {right})"),
            Expr::Sub(left, right) => write!(formatter, "({left} - {right})"),
            Expr::Equals(left, right) => write!(formatter, "({left} == {right})"),
            Expr::NotEquals(left, right) => write!(formatter, "({left} != {right})"),
            Expr::GreaterThan(left, right) => write!(formatter, "({left} > {right})"),
            Expr::GreaterThanEquals(left, right) => write!(formatter, "({left} >= {right})"),
            Expr::LessThan(left, right) => write!(formatter, "({left} < {right})"),
            Expr::LessThanEquals(left, right) => write!(formatter, "({left} <= {right})"),
            Expr::And(left, right) => write!(formatter, "({left} and {right})"),
            Expr::Or(left, right) => write!(formatter, "({left} or {right})"),
            Expr::Contains(_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod when_displaying_expr {
    use super::*;

    #[test]
    fn should_display_value() {
        assert_eq!(Expr::value(5).to_string(), "i5");
    }

    #[test]
    fn should_display_mult_div_add_sub() {
        assert_eq!(
            Expr::mult(
                Expr::add(Expr::value(3), Expr::value(4)),
                Expr::div(Expr::value(5), Expr::sub(Expr::value(6), Expr::value(7)))
            )
            .to_string(),
            "((i3 + i4) * (i5 / (i6 - i7)))"
        );
    }

    #[test]
    fn should_display_eq_neq_gte_gt_lte_lt() {
        assert_eq!(
            Expr::eq(
                Expr::neq(Expr::value(3), Expr::lte(Expr::value(4), Expr::value(9))),
                Expr::gte(
                    Expr::lt(Expr::value(5), Expr::value(8)),
                    Expr::gt(Expr::value(6), Expr::value(7))
                )
            )
            .to_string(),
            "((i3 != (i4 <= i9)) == ((i5 < i8) >= (i6 > i7)))"
        );
    }
}
