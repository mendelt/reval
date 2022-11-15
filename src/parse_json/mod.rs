use crate::{expr::Expr, ruleset::Rule};
use serde::{Deserialize, Serialize};
use serde_json::Error;

pub fn parse(input: &str) -> Result<Rule, Error> {
    serde_json::from_str::<ParseRule>(input).map(Into::<Rule>::into)
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
    Bool(bool),
    // Map(HashMap<String, ParseExpr>),
    // Vec(Vec<ParseExpr>),
    Ref(String),
    Idx(Box<ParseExpr>, Box<ParseExpr>),
    Not(Box<ParseExpr>),
    Mult(Vec<ParseExpr>),
    Div(Box<ParseExpr>, Box<ParseExpr>),
    Add(Vec<ParseExpr>),
    Sub(Box<ParseExpr>, Box<ParseExpr>),
    Eq(Box<ParseExpr>, Box<ParseExpr>),
    Neq(Box<ParseExpr>, Box<ParseExpr>),
    Gt(Box<ParseExpr>, Box<ParseExpr>),
    Gte(Box<ParseExpr>, Box<ParseExpr>),
    Lt(Box<ParseExpr>, Box<ParseExpr>),
    Lte(Box<ParseExpr>, Box<ParseExpr>),
    And(Vec<ParseExpr>),
    Or(Vec<ParseExpr>),
}

impl TryFrom<ParseExpr> for Expr {
    type Error = Error;

    fn try_from(value: ParseExpr) -> Result<Self, Self::Error> {
        Ok(match value {
            ParseExpr::String(value) => Expr::Value(value.into()),
            ParseExpr::Int(value) => Expr::Value(value.into()),
            ParseExpr::Float(value) => Expr::Value(value.into()),
            ParseExpr::Bool(value) => Expr::Value(value.into()),
            // ParseExpr::Map(value) => Expr::Value(Value::Map(value.into_iter().map(|(key, value)| (key, value.into())).collect())),
            // ParseExpr::Vec(_) => todo!(),
            ParseExpr::Ref(name) => Expr::Reference(name),
            ParseExpr::Idx(left, right) => Expr::index((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Not(value) => Expr::not((*value).try_into()?),
            ParseExpr::Mult(exprs) => operands(exprs, Expr::mult)?,
            ParseExpr::Div(left, right) => Expr::div((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Add(exprs) => operands(exprs, Expr::add)?,
            ParseExpr::Sub(left, right) => Expr::sub((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Eq(left, right) => Expr::eq((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Neq(left, right) => Expr::neq((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Gt(left, right) => Expr::gt((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Gte(left, right) => Expr::gte((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Lt(left, right) => Expr::lt((*left).try_into()?, (*right).try_into()?),
            ParseExpr::Lte(left, right) => Expr::lte((*left).try_into()?, (*right).try_into()?),
            ParseExpr::And(exprs) => operands(exprs, Expr::and)?,
            ParseExpr::Or(exprs) => operands(exprs, Expr::or)?,
        })
    }
}

/// Recursively destructure a parse expression with a list of sub-expressions into a nested Expr
fn operands(
    expressions: Vec<ParseExpr>,
    operator: impl Fn(Expr, Expr) -> Expr,
) -> Result<Expr, Error> {
    if expressions.len() < 2 {
        Err(Error::"Invalid expression: too few operands");
    } else if expressions.len() == 2 {
        Ok(operator(expressions[0].try_into()?, expressions[1].try_into()?)
    } else {
        Ok(operator(
            expressions[0].try_into()?,
            destr_parse_expr(expressions, operator), // TODO: use tail of expressions
        ))
    }

    // match expressions {
    //     [left, right] => operator((*left).into(), (*right).into()),
    //     [head, tail @ ..] => operator((*head).into(), destr_parse_expr(tail, operator)),
    //     _ => panic!("Stuff"),
    // }
}

#[cfg(test)]
mod when_parsing_json_expr {
    use super::*;
    use crate::expr::Expr;

    #[test]
    fn should_parse_string_value() {
        assert_eq!(
            parse(r#"{"name": "testrule", "expr": {"string": "test"}}"#).unwrap(),
            Rule::new("testrule", Expr::value("test"))
        );
    }

    #[test]
    fn should_parse_add_expr() {
        assert_eq!(
            parse(r#"{"name": "testrule", "expr": {"add": [{"int": 4}, {"int": 3}]}}"#).unwrap(),
            Rule::new("testrule", Expr::add(Expr::value(4), Expr::value(3)))
        );
    }
}
