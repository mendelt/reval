use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Unary(UnaryOperator, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(self) -> Value {
        match self {
            Expr::Value(value) => value,
            Expr::Unary(..) => todo!(),
            Expr::Binary(..) => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Sqrt,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    Div,
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Index,
    Max,
    Min,
}

