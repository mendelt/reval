use crate::expr::Expr;
use crate::parse::value::value;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    error::ParseError, IResult, Parser,
};

pub fn expr(input: &str) -> IResult<&str, Expr> {
    mult_expr(input)
}

fn mult_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(div_expr, tag("*"), mult_expr),
            |(left, right)| Expr::Mult(Box::new(left), Box::new(right)),
        ),
        div_expr,
    ))(input)
}

fn div_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(add_expr, tag("/"), div_expr),
            |(left, right)| Expr::Div(Box::new(left), Box::new(right)),
        ),
        add_expr,
    ))(input)
}

fn add_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(sub_expr, tag("+"), add_expr),
            |(left, right)| Expr::Add(Box::new(left), Box::new(right)),
        ),
        sub_expr,
    ))(input)
}

fn sub_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(value_expr, tag("-"), add_expr),
            |(left, right)| Expr::Sub(Box::new(left), Box::new(right)),
        ),
        value_expr,
    ))(input)
}

fn value_expr(input: &str) -> IResult<&str, Expr> {
    map(value, Expr::Value)(input)
}

pub fn binary_expr<'a, O1, O2, O3, E: ParseError<&'a str>, F, G, H>(
    mut left: F,
    mut sep: G,
    mut right: H,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O1, O3), E>
where
    F: Parser<&'a str, O1, E>,
    G: Parser<&'a str, O2, E>,
    H: Parser<&'a str, O3, E>,
{
    move |input: &'a str| {
        let (input, _) = multispace0(input)?;
        let (input, o1) = left.parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = sep.parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, o2) = right.parse(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, (o1, o2)))
        // right.parse(input).map(|(i, o2)| (i, (o1, o2)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn should_parse_mult() {
        let (rest, parsed) = expr("1*4").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Mult(
                Box::new(Expr::Value(Value::Int(1))),
                Box::new(Expr::Value(Value::Int(4))),
            )
        );
    }

    #[test]
    fn should_parse_div() {
        let (rest, parsed) = expr("1/15").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Div(
                Box::new(Expr::Value(Value::Int(1))),
                Box::new(Expr::Value(Value::Int(15))),
            )
        );
    }

    #[test]
    fn should_parse_add() {
        let (rest, parsed) = expr("1 +4").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Add(
                Box::new(Expr::Value(Value::Int(1))),
                Box::new(Expr::Value(Value::Int(4))),
            ),
        );
    }

    #[test]
    fn should_parse_sub() {
        let (rest, parsed) = expr("121-4").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Sub(
                Box::new(Expr::Value(Value::Int(121))),
                Box::new(Expr::Value(Value::Int(4))),
            ),
        );
    }
}
