use super::Error;
use crate::expr::Expr;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub reval);

impl Expr {
    pub fn parse(input: &str) -> Result<Self, Error> {
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
            Expr::parse(r#""string value""#).unwrap(),
            Expr::value("string value".to_string())
        );
    }

    #[test]
    fn should_parse_escaped_quotes() {
        assert_eq!(
            Expr::parse(r#""string \" value""#).unwrap(),
            Expr::value(r#"string \" value"#.to_string())
        );
    }

    #[test]
    #[ignore]
    fn should_unescape_escaped_characters() {
        todo!(); // TODO implement this
    }

    #[test]
    fn should_not_trim_whitespace() {
        assert_eq!(
            Expr::parse(r#""  string  value ""#).unwrap(),
            Expr::value(r#"  string  value "#.to_string())
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

    #[test]
    fn should_parse_empty_vec() {
        assert_eq!(Expr::parse("[]").unwrap(), Expr::Vec(Vec::new()))
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
    fn should_parse_vec_with_expr_items() {
        assert_eq!(
            Expr::parse("[i3+i4,i12]").unwrap().to_string(),
            "[(i3 + i4), i12]"
        );
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
    use std::collections::HashMap;

    #[test]
    fn should_parse_empty_map() {
        assert_eq!(Expr::parse("{}").unwrap(), Expr::Map(HashMap::new()))
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
            Expr::Map(
                [
                    ("item1".to_string(), Expr::value(15)),
                    ("item4".to_string(), Expr::value(2.2))
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[ignore] // TODO ignored because map order varies, use btreemap for map
    #[test]
    fn should_parse_map_with_expr_items() {
        assert_eq!(
            Expr::parse("{item1:i12+i18, item3:\"string value\"}")
                .unwrap()
                .to_string(),
            "{item1: (i12 + i18), item3: \"string value\"}"
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

    #[test]
    fn should_parse_to_upper_expr() {
        assert_eq!(
            Expr::parse(r#"to_upper("String")"#).unwrap(),
            Expr::to_upper(Expr::value("String"))
        );
    }

    #[test]
    fn should_parse_to_lower_expr() {
        assert_eq!(
            Expr::parse(r#"to_lower("String")"#).unwrap(),
            Expr::to_lower(Expr::value("String"))
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
        assert_eq!(Expr::parse("i2=i8").unwrap().to_string(), "(i2 == i8)");
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
        assert_eq!(
            Expr::parse("i12>=i4==i6<=i3").unwrap().to_string(),
            "(((i12 >= i4) == i6) <= i3)"
        )
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
mod when_parsing_contains_expressions {
    use super::*;

    #[test]
    fn should_parse_contains() {
        assert_eq!(
            Expr::parse("list contains i3").unwrap(),
            Expr::contains(Expr::Reference(String::from("list")), Expr::value(3))
        );
    }

    #[test]
    fn should_not_chain_contains() {
        assert!(Expr::parse("list contains i3 contains \"value\"").is_err())
    }

    #[test]
    fn should_parse_in() {
        assert_eq!(
            Expr::parse("i3 in list").unwrap(),
            Expr::contains(Expr::Reference(String::from("list")), Expr::value(3))
        );
    }

    #[test]
    fn should_not_chain_in() {
        assert!(Expr::parse("i3 in \"value\" in list").is_err())
    }
}

#[cfg(test)]
mod when_parsing_index_expression {
    use super::*;

    #[test]
    fn should_parse_simple_vec_index() {
        assert_eq!(
            Expr::parse("ref.14").unwrap(),
            Expr::index(Expr::reff("ref"), 14.into())
        );
    }

    #[test]
    fn should_parse_vec_index_left_associatively() {
        assert_eq!(
            Expr::parse("ref.14.15").unwrap().to_string(),
            "((ref.14).15)"
        );
    }

    #[test]
    fn should_parse_simple_map_index() {
        assert_eq!(
            Expr::parse("ref.index").unwrap(),
            Expr::index(Expr::reff("ref"), "index".into())
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

#[cfg(test)]
mod when_parsing_function_call {
    use super::*;

    #[test]
    fn should_parse_function_call_with_value_param() {
        assert_eq!(
            Expr::parse("function(i34)").unwrap(),
            Expr::func("function", Expr::value(34))
        );
    }

    #[test]
    fn should_parse_function_with_expr_param() {
        assert_eq!(
            Expr::parse("function(ref_1)").unwrap(),
            Expr::func("function", Expr::reff("ref_1"))
        )
    }

    #[test]
    fn should_parse_function_with_complex_expr_param() {
        assert_eq!(
            Expr::parse("function(i1+i2)").unwrap().to_string(),
            "function((i1 + i2))"
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
mod when_parsing_comments {
    use super::*;

    #[test]
    fn should_ignore_single_line_comment() {
        assert_eq!(Expr::parse(" // comment \ni3").unwrap().to_string(), "i3");
    }

    #[test]
    fn should_ignore_end_of_line_comment() {
        assert_eq!(
            Expr::parse("i3 // comment 1 \n+i4// comment 2")
                .unwrap()
                .to_string(),
            "(i3 + i4)"
        );
    }
}
