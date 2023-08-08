//! Parse rules writting in the Reval json format

use crate::{
    expr::{Expr, Index},
    ruleset::rule::Rule,
    value::Value,
    Error,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Rule {
    /// Parse a rule written in the Reval json format
    pub fn parse_json(input: &str) -> Result<Rule, Error> {
        Ok(serde_json::from_str::<ParseRule>(input)?.into())
    }
}

impl Expr {
    /// Parse an expression written in the Reval json format
    pub fn parse_json(input: &str) -> Result<Expr, Error> {
        Ok(serde_json::from_str::<ParseExpr>(input)?.into())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ParseRule {
    name: String,
    description: Option<String>,
    expr: ParseExpr,
}

impl From<ParseRule> for Rule {
    fn from(value: ParseRule) -> Self {
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
    None,
    Map(HashMap<String, ParseExpr>),
    Vec(Vec<ParseExpr>),
    Ref(String),
    Func(String, Box<ParseExpr>),
    Idx(Box<ParseExpr>, ParseIndex),
    If(Box<ParseExpr>, Box<ParseExpr>, Box<ParseExpr>),
    Not(Box<ParseExpr>),
    Neg(Box<ParseExpr>),
    #[serde(rename = "is_some")]
    IsSome(Box<ParseExpr>),
    #[serde(rename = "is_none")]
    IsNone(Box<ParseExpr>),
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
    Contains(Box<ParseExpr>, Box<ParseExpr>),
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
}

impl From<ParseIndex> for Index {
    fn from(value: ParseIndex) -> Self {
        match value {
            ParseIndex::Map(index) => index.into(),
            ParseIndex::Vec(index) => index.into(),
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
            ParseExpr::None => Expr::none(),
            ParseExpr::Ref(name) => Expr::Reference(name),
            ParseExpr::Func(name, param) => Expr::func(name, (*param).into()),
            ParseExpr::Idx(expr, index) => Expr::index((*expr).into(), index.into()),
            ParseExpr::If(switch, left, right) => {
                Expr::iif(*switch, (*left).into(), (*right).into())
            }
            ParseExpr::Not(value) => Expr::not((*value).into()),
            ParseExpr::Neg(value) => Expr::neg((*value).into()),
            ParseExpr::IsSome(value) => Expr::is_some((*value).into()),
            ParseExpr::IsNone(value) => Expr::is_none((*value).into()),
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
            ParseExpr::Contains(list, key) => Expr::contains((*list).into(), (*key).into()),
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
mod when_parsing_rule_metadata {
    use super::*;

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
    fn should_parse_rule_expression() {
        let rule = Rule::parse_json(
            r#"{"name": "testrule", "description": "Test Rule", "expr": {"string": "test"}}"#,
        )
        .unwrap();

        assert_eq!(rule.expr(), &Expr::value("test"));
    }
}

#[cfg(test)]
mod when_parsing_value_expressions {
    use crate::expr::Expr;

    #[test]
    fn should_parse_string_value() {
        assert_eq!(
            Expr::parse_json(r#"{"string": "test"}"#)
                .unwrap()
                .to_string(),
            "\"test\""
        );
    }

    #[test]
    fn should_parse_int_value() {
        assert_eq!(Expr::parse_json(r#"{"int": 5}"#).unwrap().to_string(), "i5");
    }

    #[test]
    fn should_parse_float_value() {
        assert_eq!(
            Expr::parse_json(r#"{"float": 5.5}"#).unwrap().to_string(),
            "f5.5"
        );
    }

    #[test]
    fn should_parse_decimal_value() {
        assert_eq!(
            Expr::parse_json(r#"{"decimal": 2.02}"#)
                .unwrap()
                .to_string(),
            "d2.02"
        );
    }

    #[test]
    fn should_parse_bool_value() {
        assert_eq!(
            Expr::parse_json(r#"{"bool": true}"#).unwrap().to_string(),
            "true"
        );
    }

    #[test]
    fn should_parse_none_value() {
        assert_eq!(Expr::parse_json(r#""none""#).unwrap().to_string(), "none");
    }
}

#[cfg(test)]
mod when_parsing_single_expressions {
    use std::collections::HashMap;

    use crate::expr::{Expr, Index};

    #[test]
    fn should_parse_reference() {
        assert_eq!(
            Expr::parse_json(r#"{"ref": "the value"}"#).unwrap(),
            Expr::Reference("the value".to_string())
        );
    }

    #[test]
    fn should_parse_function_call() {
        assert_eq!(
            Expr::parse_json(r#"{"func": ["function_name", {"string": "function parameter"}]}"#)
                .unwrap(),
            Expr::func("function_name", Expr::value("function parameter"))
        );
    }

    #[test]
    fn should_parse_index_by_string() {
        assert_eq!(
            Expr::parse_json(r#"{"idx": [{"ref": "some_map_value"}, "field"]}"#).unwrap(),
            Expr::index(
                Expr::reff("some_map_value"),
                Index::Map("field".to_string())
            )
        );
    }

    #[test]
    fn should_parse_index_by_usize() {
        assert_eq!(
            Expr::parse_json(r#"{"idx": [{"ref": "some_vec_value"}, 5]}"#).unwrap(),
            Expr::index(Expr::reff("some_vec_value"), Index::Vec(5))
        );
    }

    #[test]
    fn should_parse_if_statement() {
        assert_eq!(
            Expr::parse_json(
                r#"{"if": [{"bool": true}, {"string": "true"}, {"string": "false"}]}"#
            )
            .unwrap(),
            Expr::iif(Expr::value(true), Expr::value("true"), Expr::value("false"))
        )
    }

    #[test]
    fn should_parse_not_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"not": {"bool": true}}"#).unwrap(),
            Expr::not(Expr::value(true))
        )
    }

    #[test]
    fn should_parse_neg_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"neg": {"int": 5}}"#).unwrap(),
            Expr::neg(Expr::value(5))
        )
    }

    #[test]
    fn should_parse_is_some_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"is_some": {"string": "value"}}"#).unwrap(),
            Expr::is_some(Expr::value("value"))
        )
    }

    #[test]
    fn should_parse_is_none_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"is_none": {"string": "value"}}"#).unwrap(),
            Expr::is_none(Expr::value("value"))
        )
    }

    #[test]
    fn should_parse_cint_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"cint": {"float": 3.15}}"#).unwrap(),
            Expr::int(Expr::value(3.15))
        );
    }

    #[test]
    fn should_parse_cfloat_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"cfloat": {"int": 3}}"#).unwrap(),
            Expr::float(Expr::value(3))
        );
    }

    #[test]
    fn should_parse_cdecimal_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"cdecimal": {"int": 3}}"#).unwrap(),
            Expr::dec(Expr::value(3))
        );
    }

    #[test]
    fn should_parse_vec() {
        assert_eq!(
            Expr::parse_json(r#"{"vec": [{"string": "test 1"}, {"string": "test 2"}]}"#).unwrap(),
            Expr::Vec(vec![Expr::value("test 1"), Expr::value("test 2")])
        );
    }

    #[test]
    fn should_parse_empty_vec() {
        assert_eq!(
            Expr::parse_json(r#"{"vec": []}"#).unwrap(),
            Expr::Vec(Vec::new())
        );
    }

    #[test]
    fn should_parse_map() {
        assert_eq!(
            Expr::parse_json(
                r#"{"map": {"test 1": {"string": "test 1"}, "test 2": {"string": "test 2"}}}"#
            )
            .unwrap(),
            Expr::Map(HashMap::from([
                ("test 1".to_string(), Expr::value("test 1")),
                ("test 2".to_string(), Expr::value("test 2"))
            ]))
        )
    }

    #[test]
    fn should_parse_empty_map() {
        assert_eq!(
            Expr::parse_json(r#"{"map": {}}"#).unwrap(),
            Expr::Map(HashMap::new())
        )
    }

    #[test]
    fn should_parse_mult_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"mult": [{"int": 4}, {"int": 3}]}"#)
                .unwrap()
                .to_string(),
            "(i4 * i3)"
        )
    }

    #[test]
    fn should_parse_div_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"div": [{"int": 4}, {"int": 3}]}"#)
                .unwrap()
                .to_string(),
            "(i4 / i3)"
        )
    }

    #[test]
    fn should_parse_add_expr() {
        assert_eq!(
            Expr::parse_json(r#"{"add": [{"int": 4}, {"int": 3}]}"#)
                .unwrap()
                .to_string(),
            "(i4 + i3)"
        );
    }

    #[test]
    fn should_parse_sub_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"sub": [{"int": 4}, {"int": 3}]}"#)
                .unwrap()
                .to_string(),
            "(i4 - i3)"
        )
    }

    #[test]
    fn should_parse_eq_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"eq": [{"bool": true}, {"bool": true}]}"#)
                .unwrap()
                .to_string(),
            "(true == true)"
        )
    }

    #[test]
    fn should_parse_neq_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"neq": [{"bool": true}, {"bool": false}]}"#)
                .unwrap()
                .to_string(),
            "(true != false)"
        )
    }

    #[test]
    fn should_parse_gt_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"gt": [{"int": 4}, {"int": 15}]}"#)
                .unwrap()
                .to_string(),
            "(i4 > i15)"
        )
    }

    #[test]
    fn should_parse_gte_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"gte": [{"int": 12}, {"int": 12}]}"#)
                .unwrap()
                .to_string(),
            "(i12 >= i12)"
        )
    }

    #[test]
    fn should_parse_lt_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"lt": [{"int": 14}, {"int": 12}]}"#)
                .unwrap()
                .to_string(),
            "(i14 < i12)"
        )
    }

    #[test]
    fn should_parse_lte_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"lte": [{"int": 80}, {"int": 3}]}"#)
                .unwrap()
                .to_string(),
            "(i80 <= i3)"
        )
    }

    #[test]
    fn should_parse_and_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"and": [{"bool": true}, {"bool": false}]}"#)
                .unwrap()
                .to_string(),
            "(true and false)"
        );
    }

    #[test]
    fn should_parse_or_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"or": [{"bool": true}, {"bool": false}]}"#).unwrap(),
            Expr::or(Expr::value(true), Expr::value(false))
        );
    }

    #[test]
    fn should_parse_contains_expression() {
        assert_eq!(
            Expr::parse_json(
                r#"{"contains": [
                    {"vec": [{"string": "test 1"}, {"string": "test 2"}]},
                    {"string": "test 2"}
                ]}"#
            )
            .unwrap(),
            Expr::contains(
                Expr::Vec(vec![Expr::value("test 1"), Expr::value("test 2")]),
                Expr::value("test 2")
            )
        );
    }
}

#[cfg(test)]
mod when_parsing_json_expr_with_variable_params {
    use crate::expr::Expr;

    #[test]
    fn should_left_associatively_parse_sub_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"sub": [{"int": 4}, {"int": 3}, {"int": 2}]}"#).unwrap(),
            Expr::sub(Expr::sub(Expr::value(4), Expr::value(3)), Expr::value(2))
        );
    }

    #[test]
    fn should_left_associatively_parse_div_expression() {
        assert_eq!(
            Expr::parse_json(r#"{"div": [{"int": 5}, {"int": 4}, {"int": 3}, {"int": 2}]}"#)
                .unwrap(),
            Expr::div(
                Expr::div(Expr::div(Expr::value(5), Expr::value(4)), Expr::value(3)),
                Expr::value(2)
            )
        );
    }

    #[test]
    fn should_parse_empty_expression_to_unit_val() {
        assert_eq!(
            Expr::parse_json(r#"{"add": []}"#).unwrap(),
            Expr::value(None)
        );
    }

    #[test]
    fn should_parse_expr_with_one_operand_to_operand() {
        assert_eq!(
            Expr::parse_json(r#"{"mult": [{"int": 4}]}"#).unwrap(),
            Expr::value(4)
        );
    }
}
