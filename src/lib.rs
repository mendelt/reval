use std::collections::hash_map::HashMap;

pub enum Expr {
    Value(Value),
    Reference(String),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

pub enum Value {
    String(String),
    Integer(i128),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}

pub enum UnaryOperator {
    Not,
    Sqrt,
}

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
}

impl Expr {
    pub fn evaluate(&self) -> Value {
        todo!()
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
