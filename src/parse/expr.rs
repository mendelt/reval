use super::Parsers;
use crate::{expr::Expr, Error};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub reval);

impl Parsers for Expr {
    type Error = Error;
    type Parseable = Expr;

    fn parse(input: &str) -> Result<Expr, Error> {
        Ok(reval::ExprParser::new().parse(input).unwrap())
    }
}

#[cfg(test)]
mod when_parsing_integer {
    use super::*;

    #[test]
    fn should_parse_integer() {
        assert_eq!(Expr::parse("i15").unwrap(), Expr::value(15));
    }

    #[test]
    fn should_parse_negative_integer() {
        assert_eq!(Expr::parse("i-6").unwrap(), Expr::value(-6))
    }
}

#[cfg(test)]
mod when_parsing_float {
    use super::*;

    #[test]
    fn should_parse_simple_float() {
        assert_eq!(Expr::parse("f5").unwrap(), Expr::value(5.0));
    }

    #[test]
    fn should_parse_factional_float() {
        assert_eq!(Expr::parse("f5.5").unwrap(), Expr::value(5.5))
    }

    #[test]
    fn should_parse_negative_float() {
        assert_eq!(Expr::parse("f-5.5").unwrap(), Expr::value(-5.5));
    }

    #[test]
    fn should_parse_exponent() {
        assert_eq!(Expr::parse("f38.0e-1").unwrap(), Expr::value(3.8))
    }
}

#[cfg(test)]
mod when_parsing_decimal {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn should_parse_simple_decimal() {
        assert_eq!(Expr::parse("d5").unwrap(), Expr::value(Decimal::new(5, 0)));
    }

    #[test]
    fn should_parse_fractional_decimal() {
        assert_eq!(
            Expr::parse("d5.5").unwrap(),
            Expr::value(Decimal::new(55, 1))
        );
    }

    #[test]
    fn should_parse_negative_decimal() {
        assert_eq!(
            Expr::parse("d-5.5").unwrap(),
            Expr::value(Decimal::new(-55, 1))
        );
    }
}

#[cfg(test)]
mod when_parsing_bool {
    use super::*;

    #[test]
    fn should_parse_true() {
        assert_eq!(Expr::parse("true").unwrap(), Expr::value(true));
    }

    #[test]
    fn should_parse_false() {
        assert_eq!(Expr::parse("false").unwrap(), Expr::value(false));
    }
}

#[cfg(test)]
mod when_parsing_string {
    // use super::*;

    // #[test]
    // fn should_parse_double_quoted_string() {
    //     assert_eq!(
    //         string_value("\"string value\"").unwrap().1,
    //         Value::String("string value".to_string())
    //     );
    // }

    // #[test]
    // fn should_ignore_single_quotes_inside_double_quoted_string() {
    //     assert_eq!(
    //         string_value("\"string 'value'\"").unwrap().1,
    //         Value::String("string 'value'".to_string())
    //     );
    // }

    // #[test]
    // fn should_parse_single_quoted_string() {
    //     assert_eq!(
    //         string_value("'string value'").unwrap().1,
    //         Value::String("string value".to_string())
    //     );
    // }

    // #[test]
    // fn should_ignore_double_quotes_inside_single_quoted_string() {
    //     assert_eq!(
    //         string_value("'string \"value\"'").unwrap().1,
    //         Value::String("string \"value\"".to_string())
    //     );
    // }
}

#[cfg(test)]
mod when_parsing_expressions {
    use super::*;

    // #[test]
    // fn should_parse_less_than_equal() {
    //     should_parse(expr("12<=4"), Expr::lte(Expr::value(12), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_less_than() {
    //     should_parse(expr("1<4"), Expr::lt(Expr::value(1), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_greater_than_equal() {
    //     should_parse(expr("1>=15"), Expr::gte(Expr::value(1), Expr::value(15)));
    // }

    // #[test]
    // fn should_parse_greater_than() {
    //     should_parse(expr("1>4"), Expr::gt(Expr::value(1), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_not_equal() {
    //     should_parse(expr("14!=8"), Expr::neq(Expr::value(14), Expr::value(8)));
    // }

    // #[test]
    // fn should_parse_equal() {
    //     should_parse(expr("2==8"), Expr::eq(Expr::value(2), Expr::value(8)));
    // }

    #[test]
    fn should_parse_add() {
        assert_eq!(
            Expr::parse("i1+i4").unwrap(),
            Expr::add(Expr::value(1), Expr::value(4))
        );
    }

    #[test]
    fn should_parse_subtraction() {
        assert_eq!(
            Expr::parse("i12-i4").unwrap(),
            Expr::sub(Expr::value(12), Expr::value(4))
        );
    }

    #[test]
    fn should_parse_add_and_sub_left_associatively() {
        assert_eq!(
            Expr::parse("i1-i15+i25").unwrap(),
            Expr::add(Expr::sub(Expr::value(1), Expr::value(15)), Expr::value(25))
        );
    }

    #[test]
    fn should_parse_mult() {
        assert_eq!(
            Expr::parse("i1*i4").unwrap(),
            Expr::mult(Expr::value(1), Expr::value(4))
        );
    }

    #[test]
    fn should_parse_div() {
        assert_eq!(
            Expr::parse("i1/i15").unwrap(),
            Expr::div(Expr::value(1), Expr::value(15))
        );
    }

    #[test]
    fn should_parse_div_and_mult_left_associatively() {
        assert_eq!(
            Expr::parse("i1/i15*i25").unwrap(),
            Expr::mult(Expr::div(Expr::value(1), Expr::value(15)), Expr::value(25))
        );
    }

    #[test]
    fn should_parse_correct_precedence() {
        assert_eq!(
            Expr::parse("i14*i128+i4/i5*i3-i1").unwrap(),
            Expr::sub(
                Expr::add(
                    Expr::mult(Expr::value(14), Expr::value(128)),
                    Expr::mult(Expr::div(Expr::value(4), Expr::value(5)), Expr::value(3)),
                ),
                Expr::value(1),
            ),
        );
    }

    #[test]
    fn should_ignore_space_and_newlines() {
        assert_eq!(
            Expr::parse("i3 * i4\r\n    +i8\r\n  ").unwrap(),
            Expr::add(Expr::mult(Expr::value(3), Expr::value(4)), Expr::value(8)),
        );
    }

    #[test]
    fn should_parse_simple_parentheses() {
        assert_eq!(Expr::parse("(i5)").unwrap(), Expr::value(5));
    }

    #[test]
    fn should_parse_expression_inside_parentheses() {
        assert_eq!(
            Expr::parse("(i1+i1)").unwrap(),
            Expr::add(Expr::value(1), Expr::value(1))
        );
    }

    // #[test]
    // fn should_override_precedence_with_parentheses() {
    //     should_parse(
    //         expr("(3+2)*(1-5)"),
    //         Expr::mult(
    //             Expr::add(Expr::value(3), Expr::value(2)),
    //             Expr::sub(Expr::value(1), Expr::value(5)),
    //         ),
    //     );
    // }

    // #[ignore]
    // #[test]
    // fn should_parse_nested_parentheses() {
    //     todo!()
    // }
}
