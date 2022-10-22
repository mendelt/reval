use crate::expr::Expr;
use crate::parse::value::value;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    error::ParseError, sequence::delimited, IResult, Parser,
};

pub fn expr(input: &str) -> IResult<&str, Expr> {
    sub_expr(input)
}

#[cfg(test)]
mod when_parsing_expressions {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_correct_precedence() {
        should_parse(
            expr("14*128+4/5*3-1"),
            Expr::sub(
                Expr::add(
                    Expr::mult(Expr::value(14), Expr::value(128)),
                    Expr::div(Expr::value(4), Expr::mult(Expr::value(5), Expr::value(3))),
                ),
                Expr::value(1),
            ),
        );
    }
}

fn sub_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(add_expr, tag("-"), add_expr),
            |(left, right)| Expr::sub(left, right),
        ),
        add_expr,
    ))(input)
}

#[cfg(test)]
mod when_parsing_sub {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_expr() {
        should_parse(
            sub_expr("121-4"),
            Expr::sub(Expr::value(121), Expr::value(4)),
        );
    }

    #[test]
    fn should_failover() {
        should_parse(add_expr("15"), Expr::value(15));
    }
}

fn add_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(div_expr, tag("+"), add_expr),
            |(left, right)| Expr::add(left, right),
        ),
        div_expr,
    ))(input)
}

#[cfg(test)]
mod when_parsing_add {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_expr() {
        should_parse(add_expr("1+4"), Expr::add(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_failover() {
        should_parse(add_expr("1"), Expr::value(1));
    }
}

fn div_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(mult_expr, tag("/"), div_expr),
            |(left, right)| Expr::div(left, right),
        ),
        mult_expr,
    ))(input)
}

#[cfg(test)]
mod when_parsing_div {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_expr() {
        should_parse(div_expr("1/15"), Expr::div(Expr::value(1), Expr::value(15)));
    }

    #[test]
    fn should_failover() {
        should_parse(div_expr("1"), Expr::value(1));
    }
}

fn mult_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            binary_expr(value_expr, tag("*"), mult_expr),
            |(left, right)| Expr::mult(left, right),
        ),
        parenthesis_expr,
    ))(input)
}

#[cfg(test)]
mod when_parsing_mult {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_expr() {
        should_parse(mult_expr("1*4"), Expr::mult(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_failover() {
        should_parse(mult_expr("1"), Expr::value(1));
    }
}

fn parenthesis_expr(input: &str) -> IResult<&str, Expr> {
    alt((delimited(tag("("), expr, tag(")")), value_expr))(input)
}

#[cfg(test)]
mod when_parsing_parentheses {
    use super::{test_util::should_parse, *};

    #[test]
    fn should_parse_expression_inside() {
        should_parse(
            parenthesis_expr("(1+1)"),
            Expr::add(Expr::value(1), Expr::value(1)),
        );
    }

    #[test]
    fn should_failover() {
        should_parse(parenthesis_expr("1"), Expr::value(1));
    }
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
    }
}

#[cfg(test)]
mod test_util {
    use crate::expr::Expr;
    use nom::IResult;

    pub fn should_parse(result: IResult<&str, Expr>, expected_expr: Expr) {
        let (rest, parsed) = result.unwrap();
        assert_eq!(parsed, expected_expr);
        assert_eq!(rest, "");
    }
}
