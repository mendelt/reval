//! This example sets up a RuleSet with a simple "age check" rule and run it
//! against a simple piece input data.

use reval::prelude::*;
use serde::Serialize;

#[tokio::main]
async fn main() {
    // The input data-type for the rules, must implement serde::Serialize so it
    // can be serialized into a `reval::Value`
    #[derive(Serialize)]
    struct Data {
        age: u16,
    }

    // Set up an "age check" rule that checks if the "age" input field is
    // greater than or equal to 21
    let rule = r#"
    {
        "name": "age check",
        "expr": {
            "gte": [
                {"ref": "age"},
                {"int": 21}
            ]
        }
    }"#;

    // Set up the ruleset builder, add the rule and build the `RuleSet`
    let ruleset = ruleset()
        .with_rule(Rule::parse_json(rule).unwrap())
        .unwrap()
        .build();

    // Set up input data
    let facts = Data { age: 16 };

    // Evaluate the ruleset on the input data and check if the rule returns
    // `false`
    for outcome in ruleset.evaluate(&facts).await.unwrap() {
        assert_eq!(outcome.value.unwrap(), false.into());
    }
}
