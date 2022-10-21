use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    NotEquals(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    GreaterThanEquals(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    LessThanEquals(Box<Expr>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

impl Expr {
    pub fn evaluate(self) -> Value {
        match self {
            Expr::Value(value) => value,
            Expr::Mult(_, _) => todo!(),
            Expr::Div(_, _) => todo!(),
            Expr::Add(_, _) => todo!(),
            Expr::Sub(_, _) => todo!(),
            Expr::Equals(_, _) => todo!(),
            Expr::NotEquals(_, _) => todo!(),
            Expr::GreaterThan(_, _) => todo!(),
            Expr::GreaterThanEquals(_, _) => todo!(),
            Expr::LessThan(_, _) => todo!(),
            Expr::LessThanEquals(_, _) => todo!(),
            Expr::Index(_, _) => todo!(),
            Expr::Not(_) => todo!(),
        }
    }
}
