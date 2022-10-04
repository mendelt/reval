use crate::value::Value;

#[derive(Clone, Debug)]
pub enum Expr {
    Value(Value),
    Reference(String),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

impl Expr {
    pub fn evaluate(self) -> Value {
        match self {
            Expr::Value(value) => value,
            Expr::Reference(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Binary(_) => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Not,
    Sqrt,
    Reference,
}

#[derive(Clone, Copy, Debug)]
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

