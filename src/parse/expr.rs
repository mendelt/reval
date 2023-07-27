use super::Parsers;
use crate::{expr::Expr, Error};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub reval);

impl Parsers for Expr {
    type Error = Error;
    type Parseable = Expr;

    fn parse(input: &str) -> Result<Expr, Error> {
        reval::ExprParser::new()
            .parse(input)
            .map_err(|error| Error::ExprParseError(error.to_string()))
    }
}

#[cfg(test)]
mod when_parsing_string {
    // use super::*;

    // #[test]
    // fn should_parse_quoted_string() {
    //     assert_eq!(
    //         string_value("\"string value\"").unwrap().1,
    //         Value::String("string value".to_string())
    //     );
    // }

    // fn should_parse_string_with_escaped_characters() {
    //     todo!()
    // }
}

#[cfg(test)]
mod when_parsing_integer {
    use super::*;

    #[test]
    fn should_parse_integer() {
        assert_eq!(Expr::parse("i15").unwrap().to_string(), "i15");
    }

    #[test]
    fn should_parse_negative_integer() {
        assert_eq!(Expr::parse("i-6").unwrap().to_string(), "i-6")
    }
}

#[cfg(test)]
mod when_parsing_float {
    use super::*;

    #[test]
    fn should_parse_simple_float() {
        assert_eq!(Expr::parse("f5").unwrap().to_string(), "f5");
    }

    #[test]
    fn should_parse_factional_float() {
        assert_eq!(Expr::parse("f5.5").unwrap().to_string(), "f5.5")
    }

    #[test]
    fn should_parse_negative_float() {
        assert_eq!(Expr::parse("f-5.5").unwrap().to_string(), "f-5.5");
    }

    #[test]
    fn should_parse_exponent() {
        assert_eq!(Expr::parse("f38.0e-1").unwrap().to_string(), "f3.8");
    }
}

#[cfg(test)]
mod when_parsing_decimal {
    use super::*;

    #[test]
    fn should_parse_simple_decimal() {
        assert_eq!(Expr::parse("d5").unwrap().to_string(), "d5");
    }

    #[test]
    fn should_parse_fractional_decimal() {
        assert_eq!(Expr::parse("d5.5").unwrap().to_string(), "d5.5");
    }

    #[test]
    fn should_parse_negative_decimal() {
        assert_eq!(Expr::parse("d-5.5").unwrap().to_string(), "d-5.5");
    }
}

#[cfg(test)]
mod when_parsing_bool {
    use super::*;

    #[test]
    fn should_parse_true() {
        assert_eq!(Expr::parse("true").unwrap().to_string(), "true");
    }

    #[test]
    fn should_parse_false() {
        assert_eq!(Expr::parse("false").unwrap().to_string(), "false");
    }
}

#[cfg(test)]
mod when_parsing_none {
    use super::*;

    #[test]
    fn should_parse_none_value() {
        assert_eq!(Expr::parse("none").unwrap().to_string(), "none");
    }
}

#[cfg(test)]
mod when_parsing_vec_value {
    use super::*;
    use crate::value::Value;

    #[ignore]
    #[test]
    fn should_parse_empty_vec() {
        assert_eq!(
            Expr::parse("[]").unwrap(),
            Expr::Value(Value::Vec(Vec::new()))
        )
    }

    #[ignore]
    #[test]
    fn should_parse_vec_items() {
        todo!()
    }

    #[ignore]
    #[test]
    fn should_parse_nested_vec() {
        todo!()
    }
}

#[cfg(test)]
mod when_parsing_map_value {
    use super::*;
    use crate::value::Value;
    use std::collections::HashMap;

    #[ignore]
    #[test]
    fn should_parse_empty_map() {
        assert_eq!(
            Expr::parse("{}").unwrap(),
            Expr::Value(Value::Map(HashMap::new()))
        )
    }

    #[ignore]
    #[test]
    fn should_parse_map_keys_and_items() {
        assert_eq!(
            Expr::parse("{item1:i15,item4:d2.2}}").unwrap().to_string(),
            "{item1 : i15, item4 : d2.2}"
        )
    }

    #[ignore]
    #[test]
    fn should_parse_nested_map() {
        todo!()
    }
}

#[cfg(test)]
mod when_parsing_comparison_expressions {
    use super::*;

    #[test]
    fn should_parse_equal() {
        assert_eq!(Expr::parse("i2==i8").unwrap().to_string(), "(i2 == i8)");
    }

    #[test]
    fn should_parse_not_equal() {
        assert_eq!(Expr::parse("i14!=i8").unwrap().to_string(), "(i14 != i8)");
    }

    #[test]
    fn should_parse_greater_than() {
        assert_eq!(Expr::parse("i1>i4").unwrap().to_string(), "(i1 > i4)");
    }

    #[test]
    fn should_parse_less_than() {
        assert_eq!(Expr::parse("i1<i4").unwrap().to_string(), "(i1 < i4)");
    }

    #[test]
    fn should_parse_greater_than_equal() {
        assert_eq!(Expr::parse("i1>=i15").unwrap().to_string(), "(i1 >= i15)");
    }

    #[test]
    fn should_parse_less_than_equal() {
        assert_eq!(Expr::parse("i12<=i4").unwrap().to_string(), "(i12 <= i4)");
    }

    #[test]
    fn should_parse_equality_operators_left_associatively() {
        // TODO:
    }
}

#[cfg(test)]
mod when_parsing_calculation_expressions {
    use super::*;

    #[test]
    fn should_parse_add() {
        assert_eq!(Expr::parse("i1+i4").unwrap().to_string(), "(i1 + i4)");
    }

    #[test]
    fn should_parse_subtraction() {
        assert_eq!(Expr::parse("i12-i4").unwrap().to_string(), "(i12 - i4)");
    }

    #[test]
    fn should_parse_add_and_sub_left_associatively() {
        assert_eq!(
            Expr::parse("i1-i15+i25").unwrap().to_string(),
            "((i1 - i15) + i25)"
        );
    }

    #[test]
    fn should_parse_mult() {
        assert_eq!(Expr::parse("i1*i4").unwrap().to_string(), "(i1 * i4)");
    }

    #[test]
    fn should_parse_div() {
        assert_eq!(Expr::parse("i1/i15").unwrap().to_string(), "(i1 / i15)");
    }

    #[test]
    fn should_parse_div_and_mult_left_associatively() {
        assert_eq!(
            Expr::parse("i1/i15*i25").unwrap().to_string(),
            "((i1 / i15) * i25)"
        );
    }
}

#[cfg(test)]
mod when_parsing_expression_precedence {
    use super::*;

    #[test]
    fn should_parse_correct_precedence() {
        // TODO: check logic against equality against sum/sub against mult/div
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
            Expr::parse("i3 * i4\r\n    +i8\r\n  ").unwrap().to_string(),
            "((i3 * i4) + i8)"
        );
    }

    #[test]
    fn should_parse_simple_parentheses() {
        assert_eq!(Expr::parse("(i5)").unwrap().to_string(), "i5");
    }

    #[test]
    fn should_parse_expression_inside_parentheses() {
        assert_eq!(Expr::parse("(i1+i1)").unwrap().to_string(), "(i1 + i1)");
    }

    #[test]
    fn should_override_precedence_with_parentheses() {
        assert_eq!(
            Expr::parse("(i3+i2)*(i1-i5)").unwrap().to_string(),
            "((i3 + i2) * (i1 - i5))",
        );
    }

    #[test]
    fn should_parse_nested_parentheses() {
        assert_eq!(
            Expr::parse("(i3+i2)*(i1-(i5!=((i7))))")
                .unwrap()
                .to_string(),
            "((i3 + i2) * (i1 - (i5 != i7)))",
        );
    }
}

#[cfg(test)]
mod when_parsing_ref_expression {
    use super::*;

    #[test]
    fn should_parse_simple_ref() {
        assert_eq!(
            Expr::parse("simple_ident").unwrap().to_string(),
            "simple_ident"
        );
    }

    #[test]
    fn should_not_parse_invalid_ident_as_ref() {
        assert!(Expr::parse("stuff&").is_err());
    }
}
