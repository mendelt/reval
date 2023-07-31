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
    use super::*;

    #[test]
    fn should_parse_quoted_string() {
        assert_eq!(
            Expr::parse("\"string value\"").unwrap(),
            Expr::value("string value".to_string())
        );
    }

    #[ignore]
    #[test]
    fn should_parse_escaped_quotes() {
        assert_eq!(
            Expr::parse("\"string \\\" \\n value\"").unwrap(),
            Expr::value("string \\\" \n value".to_string())
        );
    }

    #[ignore]
    #[test]
    fn should_unescape_escaped_characters() {
        todo!();
    }

    #[test]
    fn should_not_trim_whitespace() {
        assert_eq!(
            Expr::parse("\"  string  value\"").unwrap(),
            Expr::value("  string  value".to_string())
        );
    }
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

    #[test]
    fn should_parse_empty_vec() {
        assert_eq!(
            Expr::parse("[]").unwrap(),
            Expr::Value(Value::Vec(Vec::new()))
        )
    }

    #[test]
    fn should_parse_vec_item() {
        assert_eq!(Expr::parse("[i3]").unwrap().to_string(), "[i3]");
    }

    #[test]
    fn should_parse_vec_with_multiple_items() {
        assert_eq!(Expr::parse("[i3,i12]").unwrap().to_string(), "[i3, i12]");
    }

    #[test]
    fn should_parse_nested_vec() {
        assert_eq!(
            Expr::parse("[i3,[i4,i67]]").unwrap().to_string(),
            "[i3, [i4, i67]]"
        )
    }
}

#[cfg(test)]
mod when_parsing_map_value {
    use super::*;
    use crate::value::Value;
    use std::collections::HashMap;

    #[test]
    fn should_parse_empty_map() {
        assert_eq!(
            Expr::parse("{}").unwrap(),
            Expr::Value(Value::Map(HashMap::new()))
        )
    }

    #[test]
    fn should_parse_map_key_and_item() {
        assert_eq!(
            Expr::parse("{item1:i15}").unwrap().to_string(),
            "{item1: i15}"
        )
    }

    #[test]
    fn should_parse_map_with_multiple_keys_and_items() {
        assert_eq!(
            Expr::parse("{item1:i15,item4:f2.2}").unwrap(),
            // Compare with the raw ast here because hashmap does no preserve order so testing
            // against the formatted version is unstable right now
            Expr::Value(Value::Map(
                [
                    ("item1".to_string(), Value::Int(15)),
                    ("item4".to_string(), Value::Float(2.2))
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn should_parse_nested_map() {
        assert_eq!(
            Expr::parse("{item3:{nested1:d4}}").unwrap().to_string(),
            "{item3: {nested1: d4}}"
        )
    }
}

#[cfg(test)]
mod when_parsing_func {
    use super::*;

    #[test]
    fn should_parse_simple_not() {
        assert_eq!(
            Expr::parse("!(true)").unwrap(),
            Expr::not(Expr::value(true))
        );
    }

    #[test]
    fn should_parse_simple_neg() {
        assert_eq!(Expr::parse("-(i3)").unwrap(), Expr::neg(Expr::value(3)));
    }

    #[test]
    fn should_parse_simple_is_none() {
        assert_eq!(
            Expr::parse("is_none(none)").unwrap(),
            Expr::is_none(Expr::value(None))
        );
    }

    #[test]
    fn should_parse_simple_is_some() {
        assert_eq!(
            Expr::parse("is_some(none)").unwrap(),
            Expr::is_some(Expr::value(None))
        );
    }

    #[test]
    fn should_parse_simple_int_conversion() {
        assert_eq!(
            Expr::parse("int(\"2\")").unwrap(),
            Expr::int(Expr::value("2"))
        );
    }

    #[test]
    fn should_parse_simple_float_conversion() {
        assert_eq!(
            Expr::parse("float(\"2.2\")").unwrap(),
            Expr::float(Expr::value("2.2"))
        );
    }

    #[test]
    fn should_parse_simple_dec_conversion() {
        assert_eq!(
            Expr::parse("dec(\"2.2\")").unwrap(),
            Expr::dec(Expr::value("2.2"))
        );
    }

    #[test]
    fn should_parse_sub_expr() {
        assert_eq!(
            Expr::parse("!(i2==i3)").unwrap().to_string(),
            "!((i2 == i3))"
        );
    }
}

#[cfg(test)]
mod when_parsing_if_expression {
    use super::*;

    #[test]
    fn should_parse_simple_if_then_else() {
        assert_eq!(
            Expr::parse("if true then i1 else i2").unwrap(),
            Expr::iif(Expr::value(true), Expr::value(1), Expr::value(2))
        );
    }

    #[test]
    fn should_parse_sub_expresions() {
        assert_eq!(
            Expr::parse("if(i1==d2.2)then d4.5 + d2.3 else false == false")
                .unwrap()
                .to_string(),
            "(if (i1 == d2.2) then (d4.5 + d2.3) else (false == false))"
        );
    }
}

#[cfg(test)]
mod when_parsing_logic_expressions {
    use super::*;

    #[test]
    fn should_parse_and() {
        assert_eq!(
            Expr::parse("true and false").unwrap().to_string(),
            "(true and false)"
        );
    }

    #[test]
    fn should_parse_or() {
        assert_eq!(
            Expr::parse("true or false").unwrap().to_string(),
            "(true or false)"
        );
    }

    #[test]
    fn should_parse_logic_operators_left_left_associatively() {
        assert_eq!(
            Expr::parse("true and false or true and false")
                .unwrap()
                .to_string(),
            "(((true and false) or true) and false)"
        );
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
