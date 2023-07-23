#[cfg(test)]
mod when_parsing_integer_value {
    use super::*;

    // #[test]
    // fn should_parse_integer() {
    //     assert_eq!(value("15").unwrap().1, Value::Int(15));
    // }

    // #[test]
    // fn should_parse_negative_integer() {
    //     assert_eq!(value("-6").unwrap().1, Value::Int(-6))
    // }
}

#[cfg(test)]
mod when_parsing_float_value {
    use super::*;

    // #[test]
    // fn should_parse_simple_float() {
    //     assert_eq!(value("5.0").unwrap().1, Value::Float(5.0));
    // }

    // #[test]
    // fn should_parse_negative_float() {
    //     assert_eq!(value("-5.0").unwrap().1, Value::Float(-5.0));
    // }

    // #[test]
    // fn should_parse_exponent() {
    //     assert_eq!(value("38.0e-1").unwrap().1, Value::Float(3.8))
    // }
}

#[cfg(test)]
mod when_parsing_bool_value {
    use super::*;

    // #[test]
    // fn should_parse_true() {
    //     assert_eq!(bool_value("true").unwrap().1, Value::Bool(true));
    //     // assert_eq!(bool_value("true").unwrap().1, Value::Bool(true));
    // }

    // #[test]
    // fn should_parse_false() {
    //     assert_eq!(bool_value("false").unwrap().1, Value::Bool(false));
    // }

    // #[test]
    // fn should_not_parse_non_bool() {
    //     assert!(bool_value("stuff").is_err());
    // }
}

#[cfg(test)]
mod when_parsing_string_value {
    use super::*;

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
