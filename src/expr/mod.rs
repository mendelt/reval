mod eval;
pub mod index;
pub(crate) mod keywords;

use crate::value::Value;
pub use index::Index;
use itertools::Itertools;
use std::{collections::BTreeMap, fmt::Display};

/// The Reval expression abstract syntax tree
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value
    Value(Value),

    /// Access a parameter passed in to the expression
    Reference(String),

    /// Insert a symbol from the loaded symbol table
    Symbol(String),

    /// Evaluate a user functions by name
    Function(String, Box<Expr>),

    /// Index a dictionary or an array value
    Index(Box<Expr>, Index),

    /// Evaluates to one of two expressions based on the boolean value of the first expression
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Construct a map from expression results
    Map(BTreeMap<String, Expr>),

    /// Construct a vector from expression results
    Vec(Vec<Expr>),

    /// Invert a boolean subexpression !true evaluates to false
    Not(Box<Expr>),

    /// Invert the sign of a numerical value
    Neg(Box<Expr>),

    /// True if the expression is not None
    Some(Box<Expr>),

    /// True if the expression is none
    None(Box<Expr>),

    /// Cast numerical values to int
    Int(Box<Expr>),

    /// Cast numerical values to float
    Float(Box<Expr>),

    /// Cast numerical values to decimal
    Dec(Box<Expr>),

    /// Cast string or timestamp to datetime
    DateTime(Box<Expr>),

    /// Cast string or seconds to duration
    Duration(Box<Expr>),

    /// Multiply two subexpressions
    Mult(Box<Expr>, Box<Expr>),

    /// Divide two subexpressions
    Div(Box<Expr>, Box<Expr>),

    /// Remainder of division of two subexpressions
    Rem(Box<Expr>, Box<Expr>),

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

    /// Bitwise and operation
    BitAnd(Box<Expr>, Box<Expr>),

    /// Bitwise or operation
    BitOr(Box<Expr>, Box<Expr>),

    /// Bitwise xor operation
    BitXor(Box<Expr>, Box<Expr>),

    /// Checks if a vec contains an item or if a map contains a key
    Contains(Box<Expr>, Box<Expr>),

    /// Convert a string to upper case
    UpperCase(Box<Expr>),

    // Convert a string to lower case
    LowerCase(Box<Expr>),

    /// Trim whitespace from start and end of string
    Trim(Box<Expr>),

    /// Round a float or decimal value down by removing the fractional part
    Floor(Box<Expr>),

    /// Round a float or decimal value up or down
    Round(Box<Expr>),

    /// Remove the non-fractional part of a float or decimal value
    Fract(Box<Expr>),

    /// Extract year from a datetime or duration, or construct a duration in years
    Year(Box<Expr>),

    /// Extract month from a datetime
    Month(Box<Expr>),

    /// Extract week from a datetime or duration, or construct a duration in weeks
    Week(Box<Expr>),

    /// Extract years from a datetime or duration, or construct a duration in hours
    Day(Box<Expr>),

    /// Extract years from a datetime or duration, or construct a duration in hours
    Hour(Box<Expr>),

    /// Extract years from a datetime or duration, or construct a duration in hours
    Minute(Box<Expr>),

    /// Extract years from a datetime or duration, or construct a duration in hours
    Second(Box<Expr>),
}

impl Expr {
    /// Value expression constructor
    pub fn value(value: impl Into<Value>) -> Self {
        Expr::Value(value.into())
    }

    /// None value expression constructor
    pub fn none_value() -> Self {
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

    /// Symbol expression constructor
    pub fn symbol(name: impl ToString) -> Self {
        Expr::Symbol(name.to_string())
    }

    /// Index expression constructor
    pub fn index(value: Expr, index: Index) -> Self {
        Expr::Index(Box::new(value), index)
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
    pub fn some(expr: Expr) -> Self {
        Expr::Some(Box::new(expr))
    }

    /// IsNone epression constructor
    pub fn none(expr: Expr) -> Self {
        Expr::None(Box::new(expr))
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

    pub fn datetime(expr: Expr) -> Self {
        Expr::DateTime(Box::new(expr))
    }

    pub fn duration(expr: Expr) -> Self {
        Expr::Duration(Box::new(expr))
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

    /// Remainder-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn rem(left: Expr, right: Expr) -> Self {
        Expr::Rem(Box::new(left), Box::new(right))
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

    pub fn bitwise_and(left: Expr, right: Expr) -> Self {
        Expr::BitAnd(Box::new(left), Box::new(right))
    }

    pub fn bitwise_or(left: Expr, right: Expr) -> Self {
        Expr::BitOr(Box::new(left), Box::new(right))
    }

    pub fn bitwise_xor(left: Expr, right: Expr) -> Self {
        Expr::BitXor(Box::new(left), Box::new(right))
    }

    pub fn contains(list: Expr, key: Expr) -> Self {
        Expr::Contains(Box::new(list), Box::new(key))
    }

    pub fn uppercase(param: Expr) -> Self {
        Expr::UpperCase(Box::new(param))
    }

    pub fn lowercase(param: Expr) -> Self {
        Expr::LowerCase(Box::new(param))
    }

    pub fn trim(param: Expr) -> Self {
        Expr::Trim(Box::new(param))
    }

    pub fn round(param: Expr) -> Self {
        Expr::Round(Box::new(param))
    }

    pub fn floor(param: Expr) -> Self {
        Expr::Floor(Box::new(param))
    }

    pub fn fract(param: Expr) -> Self {
        Expr::Fract(Box::new(param))
    }

    pub fn year(param: Expr) -> Self {
        Expr::Year(Box::new(param))
    }

    pub fn month(param: Expr) -> Self {
        Expr::Month(Box::new(param))
    }

    pub fn week(param: Expr) -> Self {
        Expr::Week(Box::new(param))
    }

    pub fn day(param: Expr) -> Self {
        Expr::Day(Box::new(param))
    }

    pub fn hour(param: Expr) -> Self {
        Expr::Hour(Box::new(param))
    }

    pub fn minute(param: Expr) -> Self {
        Expr::Minute(Box::new(param))
    }

    pub fn second(param: Expr) -> Self {
        Expr::Second(Box::new(param))
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
            Expr::Reference(ident) => write!(formatter, "{ident}"),
            Expr::Symbol(ident) => write!(formatter, ":{ident}"),
            Expr::Function(ident, param) => write!(formatter, "{ident}({param})"),
            Expr::Index(left, right) => write!(formatter, "({left}.{right})"),
            Expr::If(check, true_case, false_case) => {
                write!(formatter, "(if {check} then {true_case} else {false_case})")
            }
            Expr::Vec(values) => {
                write!(
                    formatter,
                    "[{}]",
                    values.iter().map(ToString::to_string).join(", ")
                )
            }
            Expr::Map(map) => write!(
                formatter,
                "{{{}}}",
                map.iter()
                    .map(|(key, value)| format!("{key}: {value}"))
                    .join(", ")
            ),
            Expr::Not(inner) => write!(formatter, "!({inner})"),
            Expr::Neg(inner) => write!(formatter, "-({inner})"),
            Expr::Some(inner) => write!(formatter, "some({inner})"),
            Expr::None(inner) => write!(formatter, "none({inner})"),
            Expr::Int(inner) => write!(formatter, "int({inner})"),
            Expr::Float(inner) => write!(formatter, "float({inner})"),
            Expr::Dec(inner) => write!(formatter, "dec({inner})"),
            Expr::DateTime(inner) => write!(formatter, "datetime({inner})"),
            Expr::Duration(inner) => write!(formatter, "duration({inner})"),
            Expr::Mult(left, right) => write!(formatter, "({left} * {right})"),
            Expr::Div(left, right) => write!(formatter, "({left} / {right})"),
            Expr::Rem(left, right) => write!(formatter, "({left} % {right})"),
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
            Expr::BitAnd(left, right) => write!(formatter, "{left} & {right}"),
            Expr::BitOr(left, right) => write!(formatter, "{left} | {right}"),
            Expr::BitXor(left, right) => write!(formatter, "{left} ^ {right}"),
            Expr::Contains(left, right) => write!(formatter, "({left} contains {right})"),
            Expr::UpperCase(param) => write!(formatter, "uppercase({param})"),
            Expr::LowerCase(param) => write!(formatter, "lowercase({param})"),
            Expr::Trim(param) => write!(formatter, "trim({param})"),
            Expr::Floor(param) => write!(formatter, "floor({param})"),
            Expr::Round(param) => write!(formatter, "round({param})"),
            Expr::Fract(param) => write!(formatter, "fract({param})"),
            Expr::Year(param) => write!(formatter, "year({param})"),
            Expr::Month(param) => write!(formatter, "month({param})"),
            Expr::Week(param) => write!(formatter, "week({param})"),
            Expr::Day(param) => write!(formatter, "day({param})"),
            Expr::Hour(param) => write!(formatter, "hour({param})"),
            Expr::Minute(param) => write!(formatter, "minute({param})"),
            Expr::Second(param) => write!(formatter, "second({param})"),
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
                Expr::div(
                    Expr::value(5),
                    Expr::sub(Expr::value(6), Expr::rem(Expr::value(7), Expr::value(8)))
                )
            )
            .to_string(),
            "((i3 + i4) * (i5 / (i6 - (i7 % i8))))"
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
