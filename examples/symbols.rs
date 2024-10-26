//! This example shows how to load and use symbols, small named expressions that can be reused between rules

use reval::prelude::*;

#[tokio::main]
async fn main() {
    // Set up a simple rule that passes the decimal value out without change
    let rule = "
// decimal
:symbol * :symbol
";

    // Set up the ruleset builder, add the rule and build the `RuleSet`
    let ruleset = ruleset()
        .with_symbol("symbol", Expr::parse("i2").unwrap())
        .with_rule(Rule::parse(rule).unwrap())
        .unwrap()
        .build();

    // Evaluate the ruleset on the input data and check if the rule returns
    // `false`
    for outcome in ruleset.evaluate(&()).await.unwrap() {
        assert_eq!(outcome.value.unwrap(), 4.into());
    }
}
