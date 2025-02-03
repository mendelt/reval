//! Test loading and evaluating with symbols

use reval::prelude::*;

#[tokio::test]
async fn should_evaluate_simple_symbol() {
    let ruleset = ruleset()
        .with_symbol("symbol_name", 4.into())
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
        .with_symbol("symbol1", 4.into())
        .with_symbol("symbol2", 8.into())
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
