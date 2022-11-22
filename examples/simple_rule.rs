use reval::prelude::*;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let rule =
        Rule::parse_json(r#"{"name": "age check", "expr": {"gt": [{"ref": "age"}, {"int": 21}]}}"#)
            .unwrap();

    let ruleset = ruleset().with_rule(rule).build();

    #[derive(Serialize)]
    struct Data {
        age: u16,
    }
    let facts = Data { age: 16 };

    assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![false.into()]);
}
