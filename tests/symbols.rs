//! Test loading and evaluating with symbols

use reval::prelude::*;

#[tokio::test]
async fn should_evaluate_simple_symbol() {
    let ruleset = ruleset()
        .with_symbol("symbol_name", Expr::parse("i4").unwrap())
        .with_rule(
            Rule::parse(
                r#"
// TestRule
:symbol_name
"#,
            )
            .unwrap(),
        )
        .unwrap()
        .build();
    assert_eq!(
        ruleset.evaluate(&()).await.unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .clone(),
        4.into()
    );
}

#[tokio::test]
async fn should_evaluate_complex_symbol_expression() {
    let ruleset = ruleset()
        .with_symbol("symbol1", Expr::parse("i4").unwrap())
        .with_symbol("symbol2", Expr::parse("i8").unwrap())
        .with_rule(
            Rule::parse(
                r#"
// TestRule
:symbol1 + :symbol2
"#,
            )
            .unwrap(),
        )
        .unwrap()
        .build();
    assert_eq!(
        ruleset.evaluate(&()).await.unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .clone(),
        12.into()
    );
}

#[tokio::test]
async fn should_evaluate_complex_expression_in_symbol() {
    let ruleset = ruleset()
        .with_symbol("symbol1", Expr::parse("i4 * i2").unwrap())
        .with_symbol("symbol2", Expr::parse("i15").unwrap())
        .with_rule(
            Rule::parse(
                r#"
// TestRule
:symbol1 + :symbol2
"#,
            )
            .unwrap(),
        )
        .unwrap()
        .build();
    assert_eq!(
        ruleset.evaluate(&()).await.unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .clone(),
        23.into()
    );
}

#[tokio::test]
async fn should_evaluate_symbol_reference_in_symbol() {
    let ruleset = ruleset()
        .with_symbol("symbol1", Expr::parse("i4").unwrap())
        .with_symbol("symbol2", Expr::parse(":symbol1").unwrap())
        .with_rule(
            Rule::parse(
                r#"
// TestRule
:symbol1 + :symbol2
"#,
            )
            .unwrap(),
        )
        .unwrap()
        .build();
    assert_eq!(
        ruleset.evaluate(&()).await.unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .clone(),
        8.into()
    );
}
