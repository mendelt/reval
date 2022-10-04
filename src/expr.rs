use crate::value::Value;



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

pub enum UnaryOperator {
    Not,
    Sqrt,
    Reference,
}

#[derive(Clone, Copy)]
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

