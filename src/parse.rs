use crate::{
    expr::Expr,
    value::{Number, Value},
};

pub fn parse(_string: &str) -> Expr {
    Expr::Value(Value::Number(Number::Int(0)))
}
