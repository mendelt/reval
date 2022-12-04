//! Parse rules writting in the Reval json format

use crate::{expr::Expr, ruleset::rule::Rule};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;

impl Rule {
    /// Parse a rule written in the Reval json format
    pub fn parse_json(input: &str) -> Result<Rule, Error> {
        serde_json::from_str::<ParseRule>(input).map(Into::<Rule>::into)
    }
}

impl Expr {
    /// Parse an expression written in the Reval json format
    pub fn parse_json(input: &str) -> Result<Expr, Error> {
        serde_json::from_str::<ParseExpr>(input).map(Into::<Expr>::into)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ParseRule {
    name: String,
    expr: ParseExpr,
}

impl From<ParseRule> for Rule {
    fn from(value: ParseRule) -> Rule {
        Rule::new(value.name, value.expr.into())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ParseExpr {
    String(String),
    Int(i128),
    Float(f64),
    Decimal(Decimal),
    Bool(bool),
    Map(HashMap<String, ParseExpr>),
    Vec(Vec<ParseExpr>),
    Ref(String),
    Func(String, Box<ParseExpr>),
    Idx(Box<ParseExpr>, Box<ParseExpr>),
    Not(Box<ParseExpr>),
    Neg(Box<ParseExpr>),
    CInt(Box<ParseExpr>),
    CFloat(Box<ParseExpr>),
    CDecimal(Box<ParseExpr>),
    Mult(Box<ParseExpr>, Box<ParseExpr>),
    Div(Box<ParseExpr>, Box<ParseExpr>),
    Add(Box<ParseExpr>, Box<ParseExpr>),
    Sub(Box<ParseExpr>, Box<ParseExpr>),
    Eq(Box<ParseExpr>, Box<ParseExpr>),
    Neq(Box<ParseExpr>, Box<ParseExpr>),
    Gt(Box<ParseExpr>, Box<ParseExpr>),
    Gte(Box<ParseExpr>, Box<ParseExpr>),
    Lt(Box<ParseExpr>, Box<ParseExpr>),
    Lte(Box<ParseExpr>, Box<ParseExpr>),
    And(Box<ParseExpr>, Box<ParseExpr>),
    Or(Box<ParseExpr>, Box<ParseExpr>),
}

impl From<ParseExpr> for Expr {
    fn from(value: ParseExpr) -> Expr {
        match value {
            ParseExpr::String(value) => Expr::Value(value.into()),
            ParseExpr::Int(value) => Expr::Value(value.into()),
            ParseExpr::Float(value) => Expr::Value(value.into()),
            ParseExpr::Decimal(value) => Expr::Value(value.into()),
            ParseExpr::Bool(value) => Expr::Value(value.into()),
            ParseExpr::Ref(name) => Expr::Reference(name),
            ParseExpr::Func(name, param) => Expr::func(name, (*param).into()),
            ParseExpr::Idx(left, right) => Expr::index((*left).into(), (*right).into()),
            ParseExpr::Not(value) => Expr::not((*value).into()),
            ParseExpr::Neg(value) => Expr::neg((*value).into()),
            ParseExpr::CInt(value) => Expr::int((*value).into()),
            ParseExpr::CFloat(value) => Expr::float((*value).into()),
            ParseExpr::CDecimal(value) => Expr::dec((*value).into()),
            ParseExpr::Map(map) => map_expr(map),
            ParseExpr::Vec(vec) => vec_expr(vec),
            ParseExpr::Mult(left, right) => Expr::mult((*left).into(), (*right).into()),
            ParseExpr::Div(left, right) => Expr::div((*left).into(), (*right).into()),
            ParseExpr::Add(left, right) => Expr::add((*left).into(), (*right).into()),
            ParseExpr::Sub(left, right) => Expr::sub((*left).into(), (*right).into()),
            ParseExpr::Eq(left, right) => Expr::eq((*left).into(), (*right).into()),
            ParseExpr::Neq(left, right) => Expr::neq((*left).into(), (*right).into()),
            ParseExpr::Gt(left, right) => Expr::gt((*left).into(), (*right).into()),
            ParseExpr::Gte(left, right) => Expr::gte((*left).into(), (*right).into()),
            ParseExpr::Lt(left, right) => Expr::lt((*left).into(), (*right).into()),
            ParseExpr::Lte(left, right) => Expr::lte((*left).into(), (*right).into()),
            ParseExpr::And(left, right) => Expr::and((*left).into(), (*right).into()),
            ParseExpr::Or(left, right) => Expr::or((*left).into(), (*right).into()),
        }
    }
}

/// Helper function that recursively creates a map value from a hashmap of ParseExpr
fn map_expr(map: HashMap<String, ParseExpr>) -> Expr {
    let hashmap: HashMap<String, Expr> = map
        .into_iter()
        .map(|(key, expr)| (key, Expr::from(expr)))
        .collect();

    Expr::Map(hashmap)
}

fn vec_expr(vec: Vec<ParseExpr>) -> Expr {
    Expr::Vec(vec.into_iter().map(|expr| expr.into()).collect())
}

#[cfg(test)]
mod when_parsing_json_expr {
    use super::*;
    use crate::expr::Expr;

    #[test]
    fn should_parse_string_value() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"string": "test"}}"#).unwrap(),
            Rule::new("testrule", Expr::value("test"))
        );
    }

    #[test]
    fn should_parse_add_expr() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"add": [{"int": 4}, {"int": 3}]}}"#)
                .unwrap(),
            Rule::new("testrule", Expr::add(Expr::value(4), Expr::value(3)))
        );
    }
}
