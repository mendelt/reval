//! Parse rules writting in the Reval json format

use crate::{expr::Expr, ruleset::rule::Rule, value::Value};
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
    description: Option<String>,
    expr: ParseExpr,
}

impl From<ParseRule> for Rule {
    fn from(value: ParseRule) -> Rule {
        Rule::new(&value.name, value.description, value.expr.into())
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
    Idx(Box<ParseExpr>, ParseIndex),
    If(Box<ParseExpr>, Box<ParseExpr>, Box<ParseExpr>),
    Not(Box<ParseExpr>),
    Neg(Box<ParseExpr>),
    CInt(Box<ParseExpr>),
    CFloat(Box<ParseExpr>),
    CDecimal(Box<ParseExpr>),
    Mult(Vec<ParseExpr>),
    Div(Vec<ParseExpr>),
    Add(Vec<ParseExpr>),
    Sub(Vec<ParseExpr>),
    Eq(Box<ParseExpr>, Box<ParseExpr>),
    Neq(Box<ParseExpr>, Box<ParseExpr>),
    Gt(Box<ParseExpr>, Box<ParseExpr>),
    Gte(Box<ParseExpr>, Box<ParseExpr>),
    Lt(Box<ParseExpr>, Box<ParseExpr>),
    Lte(Box<ParseExpr>, Box<ParseExpr>),
    And(Vec<ParseExpr>),
    Or(Vec<ParseExpr>),
}

/// Index type for parsing an index into a map or vec, can be a String, a usize
/// or an Expr that will be evaluated into an index value
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum ParseIndex {
    /// String index to index map fields
    Map(String),

    /// Usize index to index vec elements
    Vec(usize),

    /// An expression that can be evaluated into an index
    Expr(Box<ParseExpr>),
}

impl From<ParseIndex> for Expr {
    fn from(value: ParseIndex) -> Self {
        match value {
            ParseIndex::Map(index) => Expr::value(index),
            ParseIndex::Vec(index) => Expr::value(index),
            ParseIndex::Expr(expr) => (*expr).into(),
        }
    }
}

impl From<ParseExpr> for Expr {
    fn from(value: ParseExpr) -> Self {
        match value {
            ParseExpr::String(value) => Expr::Value(value.into()),
            ParseExpr::Int(value) => Expr::Value(value.into()),
            ParseExpr::Float(value) => Expr::Value(value.into()),
            ParseExpr::Decimal(value) => Expr::Value(value.into()),
            ParseExpr::Bool(value) => Expr::Value(value.into()),
            ParseExpr::Ref(name) => Expr::Reference(name),
            ParseExpr::Func(name, param) => Expr::func(name, (*param).into()),
            ParseExpr::Idx(expr, index) => Expr::index((*expr).into(), index.into()),
            ParseExpr::If(switch, left, right) => {
                Expr::iif(*switch, (*left).into(), (*right).into())
            }
            ParseExpr::Not(value) => Expr::not((*value).into()),
            ParseExpr::Neg(value) => Expr::neg((*value).into()),
            ParseExpr::CInt(value) => Expr::int((*value).into()),
            ParseExpr::CFloat(value) => Expr::float((*value).into()),
            ParseExpr::CDecimal(value) => Expr::dec((*value).into()),
            ParseExpr::Map(map) => map_expr(map),
            ParseExpr::Vec(vec) => vec_expr(vec),
            ParseExpr::Mult(params) => operands(params, Expr::mult),
            ParseExpr::Div(params) => operands(params, Expr::div),
            ParseExpr::Add(params) => operands(params, Expr::add),
            ParseExpr::Sub(params) => operands(params, Expr::sub),
            ParseExpr::Eq(left, right) => Expr::eq((*left).into(), (*right).into()),
            ParseExpr::Neq(left, right) => Expr::neq((*left).into(), (*right).into()),
            ParseExpr::Gt(left, right) => Expr::gt((*left).into(), (*right).into()),
            ParseExpr::Gte(left, right) => Expr::gte((*left).into(), (*right).into()),
            ParseExpr::Lt(left, right) => Expr::lt((*left).into(), (*right).into()),
            ParseExpr::Lte(left, right) => Expr::lte((*left).into(), (*right).into()),
            ParseExpr::And(params) => operands(params, Expr::and),
            ParseExpr::Or(params) => operands(params, Expr::or),
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

/// Recursively destructure a list of sub-expressions into a left associative nested Expr
fn operands(
    expressions: impl IntoIterator<Item = ParseExpr>,
    operator: impl Fn(Expr, Expr) -> Expr,
) -> Expr {
    // TODO: Make this left associative, its right-associative right now

    fn recurse(
        head: Expr,
        mut tail: impl Iterator<Item = ParseExpr>,
        operator: &impl Fn(Expr, Expr) -> Expr,
    ) -> Expr {
        match tail.next() {
            None => head,
            Some(expr) => recurse(operator(head, expr.into()), tail, operator), // Some(expr) => operator(head, recurse(expr.into(), tail, operator)),
        }
    }

    let mut tail = expressions.into_iter();

    match tail.next() {
        None => Value::None.into(),
        Some(head) => recurse(head.into(), tail, &operator),
    }
}

#[cfg(test)]
mod when_parsing_json_expr {
    use super::*;
    use crate::expr::Expr;

    #[test]
    fn should_parse_rule_name() {
        let rule = Rule::parse_json(
            r#"{"name": "testrule", "description": "Test Rule", "expr": {"string": "test"}}"#,
        )
        .unwrap();

        assert_eq!(rule.name(), "testrule");
    }

    #[test]
    fn should_parse_rule_description() {
        let rule = Rule::parse_json(
            r#"{"name": "testrule", "description": "Test Rule", "expr": {"string": "test"}}"#,
        )
        .unwrap();

        assert_eq!(rule.description(), Some("Test Rule"));
    }

    #[test]
    fn should_parse_string_value() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"string": "test"}}"#).unwrap(),
            Rule::new("testrule", None, Expr::value("test"))
        );
    }

    #[test]
    fn should_parse_add_expr() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"add": [{"int": 4}, {"int": 3}]}}"#)
                .unwrap(),
            Rule::new("testrule", None, Expr::add(Expr::value(4), Expr::value(3)))
        );
    }

    #[test]
    fn should_left_associatively_parse_sub_expr() {
        assert_eq!(
            Rule::parse_json(
                r#"{"name": "testrule", "expr": {"sub": [{"int": 4}, {"int": 3}, {"int": 2}]}}"#
            )
            .unwrap(),
            Rule::new(
                "testrule",
                None,
                Expr::sub(Expr::sub(Expr::value(4), Expr::value(3)), Expr::value(2))
            )
        );
    }

    #[test]
    fn should_parse_empty_sub_expr_to_unit_val() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"sub": []}}"#).unwrap(),
            Rule::new("testrule", None, Expr::value(None))
        );
    }

    #[test]
    fn should_parse_empty_sub_expr_with_one_operand_to_operand() {
        assert_eq!(
            Rule::parse_json(r#"{"name": "testrule", "expr": {"sub": [{"int": 4}]}}"#).unwrap(),
            Rule::new("testrule", None, Expr::value(4))
        );
    }

    #[test]
    fn should_left_associatively_parse_div_expr() {
        assert_eq!(
            Rule::parse_json(
                r#"{"name": "testrule", "expr": {"div": [{"int": 5}, {"int": 4}, {"int": 3}, {"int": 2}]}}"#
            )
            .unwrap(),
            Rule::new(
                "testrule", None,
                Expr::div(Expr::div(Expr::div(Expr::value(5), Expr::value(4)), Expr::value(3)), Expr::value(2))
            )
        );
    }

    #[test]
    fn should_index_by_string() {
        assert_eq!(
            Rule::parse_json(
                r#"{"name": "testrule", "expr": {"idx": [{"ref": "some_map_value"}, "field"]}}"#
            )
            .unwrap(),
            Rule::new(
                "testrule",
                None,
                Expr::index(Expr::reff("some_map_value"), Expr::value("field"))
            )
        );
    }

    #[test]
    fn should_index_by_usize() {
        assert_eq!(
            Rule::parse_json(
                r#"{"name": "testrule", "expr": {"idx": [{"ref": "some_vec_value"}, 5]}}"#
            )
            .unwrap(),
            Rule::new(
                "testrule",
                None,
                Expr::index(Expr::reff("some_vec_value"), Expr::value(5usize))
            )
        );
    }

    #[test]
    fn should_index_by_expression() {
        assert_eq!(
            Rule::parse_json(
                r#"{"name": "testrule", "expr": {"idx": [{"ref": "some_map_value"}, {"string": "field"}]}}"#
            )
            .unwrap(),
            Rule::new(
                "testrule",
                None,
                Expr::index(Expr::reff("some_map_value"), Expr::value("field"))
            )
        );
    }
}
