use std::collections::hash_map::HashMap;

pub enum Value {
    String(String),
    Number(Number),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}

pub enum Number {
    UInt(u128),
    Int(i128),
    Float(f64),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
