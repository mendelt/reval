//! This example shows how to extend Reval rules with `UserFunction`s
//! implemented in Rust that can be called from rules.

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

    // Set up a FakeId UserFunction that increments an integer `Value` to
    // bypass the age check
    struct FakeId;
    #[async_trait::async_trait]
    impl UserFunction for FakeId {
        async fn call(&self, param: Value) -> anyhow::Result<Value> {
            match param {
                Value::Int(age) => Ok((age + 5).into()),
                _ => Err(anyhow::anyhow!(
                    "Invalid value {:?}, expected Value::Int",
                    param
                )),
            }
        }
    }

    // Set up an "age check" rule that checks if the "age" input field is
    // greater than or equal to 21. But it first calls the `fake_id` user-
    // function.
    let rule = r#"
    {
        "name": "age check",
        "expr": {
            "gte": [
                {"func": ["fake_id", {"ref": "age"}]},
                {"int": 21}
            ]
        }
    }"#;

    // Set up the ruleset builder, add the rule, add the user-function and
    // build the `RuleSet`
    let ruleset = ruleset()
        .with_rule(Rule::parse_json(rule).unwrap())
        .with_function("fake_id", FakeId {})
        .build();

    // Set up input data
    let facts = Data { age: 16 };

    // Evaluate the ruleset on the input data and check if the rule returns
    // `true`
    assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![true.into()]);
}
