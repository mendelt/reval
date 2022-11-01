use crate::expr::Expr;
use crate::parse::value::value;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, map_res},
    error::ParseError,
    sequence::delimited,
    IResult, Parser,
};

/// Parse a full expression
pub fn expr(input: &str) -> IResult<&str, Expr> {
    lte_expr(input)
}

/// Parse less than or equals expression or failover to less than
fn lte_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(lt_expr, "<=", lte_expr, Expr::lte), lt_expr))(input)
}

/// Parse less than expression or failover to greater than or equals
fn lt_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(gte_expr, "<", lt_expr, Expr::lt), gte_expr))(input)
}

/// Parse greater than or equals expression or failover to greater than
fn gte_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(gt_expr, ">=", gte_expr, Expr::gte), gt_expr))(input)
}

/// Parse greater than expression or failover to not equal
fn gt_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(neq_expr, ">", gt_expr, Expr::gt), neq_expr))(input)
}

/// Parse not equal expression or failover to equal
fn neq_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(eq_expr, "!=", neq_expr, Expr::neq), eq_expr))(input)
}

/// Parse equal expression or failover to subtraction
fn eq_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(sub_expr, "==", eq_expr, Expr::eq), sub_expr))(input)
}

/// Parse subtraction expression or failover to addition
fn sub_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(add_expr, "-", sub_expr, Expr::sub), add_expr))(input)
}

/// Parse addition expression or failover to division
fn add_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(div_expr, "+", add_expr, Expr::add), div_expr))(input)
}

/// Parse division expression or failover to multiplication
fn div_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(mult_expr, "/", div_expr, Expr::div), mult_expr))(input)
}

/// Parse multiplication expression or faiover to parentheses
fn mult_expr(input: &str) -> IResult<&str, Expr> {
    alt((binary_expr(not_expr, "*", mult_expr, Expr::mult), not_expr))(input)
}

/// TODO: Implement negation parsing
fn not_expr(input: &str) -> IResult<&str, Expr> {
    par_expr(input) // TODO
}

/// Parse parentheses expression or failover to value
fn par_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map_res(
            delimited(tag("("), take_until_unbalanced('(', ')'), tag(")")),
            |val| expr(val).map(|parsed| parsed.1),
        ),
        idx_expr,
    ))(input)
}

fn idx_expr(input: &str) -> IResult<&str, Expr> {
    value_expr(input) // TODO
}

/// Parse value expression
fn value_expr(input: &str) -> IResult<&str, Expr> {
    map(value, Expr::Value)(input)
}

/// Helper parser to parse infix binary expression while ignoring space
/// and newlines
fn binary_expr<'a, O1, O2, E, F, G, H>(
    mut left: F,
    sep: &'static str,
    mut right: G,
    expr: H,
) -> impl FnMut(&'a str) -> IResult<&'a str, Expr, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O1, E>,
    G: Parser<&'a str, O2, E>,
    H: Fn(O1, O2) -> Expr,
{
    move |input: &'a str| {
        let (input, _) = multispace0(input)?;
        let (input, o1) = left.parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(sep)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, o2) = right.parse(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, expr(o1, o2)))
    }
}

pub fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(nom::Err::Error(nom::error::Error::from_error_kind(
                i,
                nom::error::ErrorKind::TakeUntil,
            )))
        }
    }
}

#[cfg(test)]
mod when_parsing_expressions {
    use super::*;

    #[test]
    fn should_parse_less_than_equal() {
        should_parse(expr("12<=4"), Expr::lte(Expr::value(12), Expr::value(4)));
    }

    #[test]
    fn should_parse_less_than() {
        should_parse(expr("1<4"), Expr::lt(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_parse_greater_than_equal() {
        should_parse(expr("1>=15"), Expr::gte(Expr::value(1), Expr::value(15)));
    }

    #[test]
    fn should_parse_greater_than() {
        should_parse(expr("1>4"), Expr::gt(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_parse_not_equal() {
        should_parse(expr("14!=8"), Expr::neq(Expr::value(14), Expr::value(8)));
    }

    #[test]
    fn should_parse_equal() {
        should_parse(expr("2==8"), Expr::eq(Expr::value(2), Expr::value(8)));
    }

    #[test]
    fn should_parse_subtraction() {
        should_parse(expr("12-4"), Expr::sub(Expr::value(12), Expr::value(4)));
    }

    #[test]
    fn should_parse_addition() {
        should_parse(expr("1+4"), Expr::add(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_parse_division() {
        should_parse(expr("1/15"), Expr::div(Expr::value(1), Expr::value(15)));
    }

    #[test]
    fn should_parse_multiplication() {
        should_parse(expr("1*4"), Expr::mult(Expr::value(1), Expr::value(4)));
    }

    #[test]
    fn should_parse_expression_inside_parentheses() {
        should_parse(expr("(1+1)"), Expr::add(Expr::value(1), Expr::value(1)));
    }

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

    #[test]
    fn should_ignore_space_and_newlines() {
        should_parse(
            expr("3 * 4\r\n    + 8\r\n  "),
            Expr::add(Expr::mult(Expr::value(3), Expr::value(4)), Expr::value(8)),
        );
    }

    #[test]
    fn should_override_precedence_with_parentheses() {
        should_parse(
            expr("(3+2)*(1-5)"),
            Expr::mult(
                Expr::add(Expr::value(3), Expr::value(2)),
                Expr::sub(Expr::value(1), Expr::value(5)),
            ),
        );
    }

    #[ignore]
    #[test]
    fn should_parse_nested_parentheses() {
        todo!()
    }

    /// Helper function to test parsing, checks if the Result of a parse-
    /// operation is not an error, if there is no rest and if the parsed
    /// expression equals the expected value.
    fn should_parse(result: IResult<&str, Expr>, expected_expr: Expr) {
        let (rest, parsed) = result.unwrap();
        assert_eq!(parsed, expected_expr);
        assert_eq!(rest, "");
    }
}
