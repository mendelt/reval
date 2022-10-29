use crate::expr::Expr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
            ParseExpr::Bool(value) => Expr::Value(value.into()),
            // ParseExpr::Map(value) => Expr::Value(Value::Map(value.into_iter().map(|(key, value)| (key, value.into())).collect())),
            // ParseExpr::Vec(_) => todo!(),
            ParseExpr::Ref(name) => Expr::Reference(name),
            ParseExpr::Idx(left, right) => Expr::index((*left).into(), (*right).into()),
            ParseExpr::Not(value) => Expr::not((*value).into()),
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

#[cfg(test)]
mod when_parsing_json_expr {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn should_parse() {
        assert_eq!(&to_string(&ParseExpr::String("".to_string())).unwrap(), "");
    }

    #[test]
    fn should_serialize_add() {
        assert_eq!(
            &to_string(&ParseExpr::Add(
                Box::new(ParseExpr::Int(4)),
                Box::new(ParseExpr::Int(3))
            ))
            .unwrap(),
            ""
        )
    }
}
