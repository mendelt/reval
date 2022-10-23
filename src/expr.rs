use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A literal value
    Value(Value),

    /// A reference to a value
    Reference(String),

    /// Indexes a dictionary or an array value
    Index(Box<Expr>, Box<Expr>),

    /// Negates a subexpression !true evaluates to false
    Not(Box<Expr>),

    /// Multiply two subexpressions
    Mult(Box<Expr>, Box<Expr>),

    /// Divide two subexpressions
    Div(Box<Expr>, Box<Expr>),

    /// Add two subexpressions
    Add(Box<Expr>, Box<Expr>),

    /// Subtract two subexpressions
    Sub(Box<Expr>, Box<Expr>),

    /// Equates two subexpressions
    Equals(Box<Expr>, Box<Expr>),

    /// Inverse equation of two subexpressions
    NotEquals(Box<Expr>, Box<Expr>),

    /// Checks if one of two subexpressions is greater than the other
    GreaterThan(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpressions is greater than or equal to the other
    GreaterThanEquals(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpression is less than the other
    LessThan(Box<Expr>, Box<Expr>),

    /// Checks if the left subexpression is less than or equal to the other
    LessThanEquals(Box<Expr>, Box<Expr>),

    /// And operation on two subexpressions
    And(Box<Expr>, Box<Expr>),

    /// Or operation on two subexpressions
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(self) -> Value {
        match self {
            Expr::Value(value) => value,
            Expr::Reference(_) => todo!(),
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
            Expr::And(_, _) => todo!(),
            Expr::Or(_, _) => todo!(),
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
