# Reval &emsp; [![Build Status](https://github.com/mendelt/reval/workflows/Build/badge.svg)](https://github.com/mendelt/reval/actions?query=workflow%3ABuild+event%3Apush+branch%3Amain)

<!-- cargo-rdme start -->

Reval, short for Rust Evaluator is a light-weight expression evaluator library. It can be used as a rules-engine or in other situations where simple user expressions need to be evaluated.

Expressions or rules can be written in a simple DSL or in a json format that is directly parsed into Reval expression AST objects. For now the Reval DSL parser is experimental but eventually this should replace json rules.

Expressions can be evaluated against input data to produce output data. The input and output data is passed in and out of the rules as a `reval::Value` object which can contain simple data or more complex structures like maps or vectors.
Data can even be nested so complex data can be passed into Reval expressions.

To make it easy to construct input data from your own datatypes the Reval crate implements a serde serializer for Value types. So any type that implements `serde::Serialize` can be serialized into a `reval::Value` without writing any code.

```rust
use reval::{value::Value, value::ser::ValueSerializer, parse_json::parse};
use serde::Serialize;

let rule = parse(r#"{"name": "age check", "expr": {"gt": [{"ref": "age"}, {"int": 21}]}}"#).unwrap();

#[derive(Serialize)]
struct Data { age: u16 }

let facts = Data {age: 16}.serialize(ValueSerializer).unwrap();

assert_eq!(rule.evaluate(&facts).unwrap(), false.into());

```

<!-- cargo-rdme end -->

*version: {{version}}*
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
