//! This example shows how to pass in Decimal values and how to use them in a rule
//! This is a bit of a workaround/hack

use reval::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[tokio::main]
async fn main() {
    // Pass in a data-type containing a Decimal field
    #[derive(Serialize)]
    struct Data {
        amount: Decimal,
    }

    // Set up a simple rule that passes the decimal value out without change
    let rule = r"
// decimal
// decimals are passed in as string (because of serde limitations) so we need to convert it using the `dec` function
dec(amount) > d900
";

    // Set up the ruleset builder, add the rule and build the `RuleSet`
    let ruleset = ruleset()
        .with_rule(Rule::parse(rule).unwrap())
        .unwrap()
        .build();

    // Set up input data
    let facts = Data {
        amount: Decimal::new(1000, 0),
    };

    // Evaluate the ruleset on the input data and check if the rule returns
    // `false`
    for outcome in ruleset.evaluate(&facts).await.unwrap() {
        assert_eq!(outcome.value.unwrap(), Value::Bool(true));
    }
}
