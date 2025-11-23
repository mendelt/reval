use crate::{
    expr::{Expr, Index},
    value::Value,
};

impl Expr {
    /// Value expression constructor
    pub fn value(value: impl Into<Value>) -> Self {
        Expr::Value(value.into())
    }

    /// None value expression constructor
    pub fn none_value() -> Self {
        Expr::Value(Value::None)
    }

    /// Function expression constructor
    pub fn func(name: impl Into<String>, param: Expr) -> Self {
        Expr::Function(name.into(), Box::new(param))
    }

    /// Reference an input value
    pub fn reff(name: impl ToString) -> Self {
        Expr::Reference(name.to_string())
    }

    /// Symbol expression constructor
    pub fn symbol(name: impl ToString) -> Self {
        Expr::Symbol(name.to_string())
    }

    /// Index expression constructor
    pub fn index(value: Expr, index: Index) -> Self {
        Expr::Index(Box::new(value), index)
    }

    /// If expression constructor
    pub fn iif(swith: impl Into<Expr>, yes: Expr, no: Expr) -> Self {
        Expr::If(Box::new(swith.into()), Box::new(yes), Box::new(no))
    }

    /// Not expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Expr) -> Self {
        Expr::Not(Box::new(expr))
    }

    /// Neg expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn neg(expr: Expr) -> Self {
        Expr::Neg(Box::new(expr))
    }

    /// IsSome epression constructor
    pub fn some(expr: Expr) -> Self {
        Expr::Some(Box::new(expr))
    }

    /// IsNone epression constructor
    pub fn none(expr: Expr) -> Self {
        Expr::None(Box::new(expr))
    }

    /// Int-cast expression constructor
    pub fn int(expr: Expr) -> Self {
        Expr::Int(Box::new(expr))
    }

    /// Float-cast expression constructor
    pub fn float(expr: Expr) -> Self {
        Expr::Float(Box::new(expr))
    }

    /// Decimal-cast expression constructor
    pub fn dec(expr: Expr) -> Self {
        Expr::Dec(Box::new(expr))
    }

    pub fn datetime(expr: Expr) -> Self {
        Expr::DateTime(Box::new(expr))
    }

    pub fn duration(expr: Expr) -> Self {
        Expr::Duration(Box::new(expr))
    }

    /// Multiply-expression constructor
    pub fn mult(left: Expr, right: Expr) -> Self {
        Expr::Mult(Box::new(left), Box::new(right))
    }

    /// Divide-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Div(Box::new(left), Box::new(right))
    }

    /// Remainder-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn rem(left: Expr, right: Expr) -> Self {
        Expr::Rem(Box::new(left), Box::new(right))
    }

    /// Add-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    /// Subtract-expression constructor
    #[allow(clippy::should_implement_trait)]
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Sub(Box::new(left), Box::new(right))
    }

    /// Equals-expression constructor
    pub fn eq(left: Expr, right: Expr) -> Self {
        Expr::Equals(Box::new(left), Box::new(right))
    }

    /// Not-equals expression constructor
    pub fn neq(left: Expr, right: Expr) -> Self {
        Expr::NotEquals(Box::new(left), Box::new(right))
    }

    /// Greater-than expression constructor
    pub fn gt(left: Expr, right: Expr) -> Self {
        Expr::GreaterThan(Box::new(left), Box::new(right))
    }

    /// Greater-than-or-equals expression constructor
    pub fn gte(left: Expr, right: Expr) -> Self {
        Expr::GreaterThanEquals(Box::new(left), Box::new(right))
    }

    /// Less-than expression constructor
    pub fn lt(left: Expr, right: Expr) -> Self {
        Expr::LessThan(Box::new(left), Box::new(right))
    }

    /// Less-than-or-equals expression constructor
    pub fn lte(left: Expr, right: Expr) -> Self {
        Expr::LessThanEquals(Box::new(left), Box::new(right))
    }

    /// And expression constructor
    pub fn and(left: Expr, right: Expr) -> Self {
        Expr::And(Box::new(left), Box::new(right))
    }

    /// Or expression constructor
    pub fn or(left: Expr, right: Expr) -> Self {
        Expr::Or(Box::new(left), Box::new(right))
    }

    pub fn bitwise_and(left: Expr, right: Expr) -> Self {
        Expr::BitAnd(Box::new(left), Box::new(right))
    }

    pub fn bitwise_or(left: Expr, right: Expr) -> Self {
        Expr::BitOr(Box::new(left), Box::new(right))
    }

    pub fn bitwise_xor(left: Expr, right: Expr) -> Self {
        Expr::BitXor(Box::new(left), Box::new(right))
    }

    pub fn contains(list: Expr, key: Expr) -> Self {
        Expr::Contains(Box::new(list), Box::new(key))
    }

    pub fn starts(prefix: Expr, string: Expr) -> Self {
        Expr::Starts(Box::new(prefix), Box::new(string))
    }

    pub fn ends(suffix: Expr, string: Expr) -> Self {
        Expr::Ends(Box::new(suffix), Box::new(string))
    }

    pub fn uppercase(param: Expr) -> Self {
        Expr::UpperCase(Box::new(param))
    }

    pub fn lowercase(param: Expr) -> Self {
        Expr::LowerCase(Box::new(param))
    }

    pub fn trim(param: Expr) -> Self {
        Expr::Trim(Box::new(param))
    }

    pub fn round(param: Expr) -> Self {
        Expr::Round(Box::new(param))
    }

    pub fn floor(param: Expr) -> Self {
        Expr::Floor(Box::new(param))
    }

    pub fn fract(param: Expr) -> Self {
        Expr::Fract(Box::new(param))
    }

    pub fn year(param: Expr) -> Self {
        Expr::Year(Box::new(param))
    }

    pub fn month(param: Expr) -> Self {
        Expr::Month(Box::new(param))
    }

    pub fn week(param: Expr) -> Self {
        Expr::Week(Box::new(param))
    }

    pub fn day(param: Expr) -> Self {
        Expr::Day(Box::new(param))
    }

    pub fn hour(param: Expr) -> Self {
        Expr::Hour(Box::new(param))
    }

    pub fn minute(param: Expr) -> Self {
        Expr::Minute(Box::new(param))
    }

    pub fn second(param: Expr) -> Self {
        Expr::Second(Box::new(param))
    }
}
