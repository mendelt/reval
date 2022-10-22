use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Index(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
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
}

impl Expr {
    pub fn evaluate(self) -> Value {
        match self {
            Expr::Value(value) => value,
            Expr::Index(_, _) => todo!(),
            Expr::Not(_) => todo!(),
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
        }
    }

    pub fn value(value: impl Into<Value>) -> Self {
        Expr::Value(value.into())
    }

    pub fn index(left: Expr, right: Expr) -> Self {
        Expr::Index(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Expr) -> Self {
        Expr::Not(Box::new(expr))
    }

    pub fn mult(left: Expr, right: Expr) -> Self {
        Expr::Mult(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Div(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Sub(Box::new(left), Box::new(right))
    }

    pub fn eq(left: Expr, right: Expr) -> Self {
        Expr::Equals(Box::new(left), Box::new(right))
    }

    pub fn neq(left: Expr, right: Expr) -> Self {
        Expr::NotEquals(Box::new(left), Box::new(right))
    }

    pub fn gt(left: Expr, right: Expr) -> Self {
        Expr::GreaterThan(Box::new(left), Box::new(right))
    }

    pub fn gte(left: Expr, right: Expr) -> Self {
        Expr::GreaterThanEquals(Box::new(left), Box::new(right))
    }

    pub fn lt(left: Expr, right: Expr) -> Self {
        Expr::LessThan(Box::new(left), Box::new(right))
    }

    pub fn lte(left: Expr, right: Expr) -> Self {
        Expr::LessThanEquals(Box::new(left), Box::new(right))
    }
}
