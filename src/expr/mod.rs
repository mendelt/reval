pub(crate) mod constructor;
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

    /// Checks if a string starts with a prefix
    Starts(Box<Expr>, Box<Expr>),

    /// Checks if a string ends with a suffix
    Ends(Box<Expr>, Box<Expr>),

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

    /// Map items in a list by evaluating an expression for each item
    ForMap(String, Box<Expr>, Box<Expr>),

    /// Filter items in a list by evaluating a predicate expression for each item
    ForFilter(String, Box<Expr>, Box<Expr>),
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
            Expr::Starts(expr, expr1) => write!(formatter, "starts({expr}, {expr1})"),
            Expr::Ends(expr, expr1) => write!(formatter, "ends({expr}, {expr1})"),
            Expr::ForFilter(bind, list, pred) => {
                write!(formatter, "for {bind} in {list} filter {pred}")
            }
            Expr::ForMap(bind, list, expr) => write!(formatter, "for {bind} in {list} map {expr}"),
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
