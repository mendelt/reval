mod eval;
pub(crate) mod keywords;

use crate::value::Value;
use std::collections::HashMap;

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
    pub fn func(name: String, param: Expr) -> Self {
        Expr::Function(name, Box::new(param))
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
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}
