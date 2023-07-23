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

    // #[test]
    // fn should_parse_subtraction() {
    //     should_parse(expr("12-4"), Expr::sub(Expr::value(12), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_addition() {
    //     should_parse(expr("1+4"), Expr::add(Expr::value(1), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_division() {
    //     should_parse(expr("1/15"), Expr::div(Expr::value(1), Expr::value(15)));
    // }

    // #[test]
    // fn should_parse_multiplication() {
    //     should_parse(expr("1*4"), Expr::mult(Expr::value(1), Expr::value(4)));
    // }

    // #[test]
    // fn should_parse_expression_inside_parentheses() {
    //     should_parse(expr("(1+1)"), Expr::add(Expr::value(1), Expr::value(1)));
    // }

    // #[test]
    // fn should_parse_correct_precedence() {
    //     should_parse(
    //         expr("14*128+4/5*3-1"),
    //         Expr::sub(
    //             Expr::add(
    //                 Expr::mult(Expr::value(14), Expr::value(128)),
    //                 Expr::div(Expr::value(4), Expr::mult(Expr::value(5), Expr::value(3))),
    //             ),
    //             Expr::value(1),
    //         ),
    //     );
    // }

    // #[test]
    // fn should_ignore_space_and_newlines() {
    //     should_parse(
    //         expr("3 * 4\r\n    + 8\r\n  "),
    //         Expr::add(Expr::mult(Expr::value(3), Expr::value(4)), Expr::value(8)),
    //     );
    // }

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

    // /// Helper function to test parsing, checks if the Result of a parse-
    // /// operation is not an error, if there is no rest and if the parsed
    // /// expression equals the expected value.
    // fn should_parse(result: IResult<&str, Expr>, expected_expr: Expr) {
    //     let (rest, parsed) = result.unwrap();
    //     assert_eq!(parsed, expected_expr);
    //     assert_eq!(rest, "");
    // }
}
