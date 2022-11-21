use reval::{prelude::*, value::ser::ValueSerializer};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let rule =
        parse_json(r#"{"name": "age check", "expr": {"gt": [{"ref": "age"}, {"int": 21}]}}"#)
            .unwrap();

    let mut ruleset = RuleSet::default();
    ruleset.add_rule(rule);

    #[derive(Serialize)]
    struct Data {
        age: u16,
    }
    let facts = Data { age: 16 }.serialize(ValueSerializer).unwrap();

    assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![false.into()]);
}
