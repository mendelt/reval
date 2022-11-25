//! Reval, short for Rust Evaluator is a light-weight expression evaluator library. It can be used as a rules-engine or in other situations where simple user expressions need to be evaluated.
//!
//! Expressions or rules can be written in a simple DSL or in a json format that is directly parsed into Reval expression AST objects. For now the Reval DSL parser is experimental but eventually this should replace json rules.
//!
//! Expressions can be evaluated against input data to produce output data. The input and output data is passed in and out of the rules as a `reval::Value` object which can contain simple data or more complex structures like maps or vectors.
//! Data can even be nested so complex data can be passed into Reval expressions.
//!
//! To make it easy to construct input data from your own datatypes the Reval crate implements a serde serializer for Value types. So any type that implements `serde::Serialize` can be serialized into a `reval::Value` without writing any code.
//!
//! This example shows how to set up a RuleSet using the builder. A simple Rule is added and a data-type is set up that is used as input for the rules;
//! ```rust
//! # tokio_test::block_on(async {
//! use reval::prelude::*;
//! use serde::Serialize;
//!
//! // The input data-type for the rules, must implement serde::Serialize so it
//! // can be serialized into a `reval::Value`
//! #[derive(Serialize)]
//! struct Data {
//!     age: u16,
//! }
//! // Set up an "age check" rule that checks if the "age" input field is
//! // greater than or equal to 21
//! let rule = r#"
//! {
//!     "name": "age check",
//!     "expr": {
//!         "gte": [
//!             {"ref": "age"},
//!             {"int": 21}
//!         ]
//!     }
//! }"#;
//! // Set up the ruleset builder, add the rule and build the `RuleSet`
//! let ruleset = ruleset().with_rule(Rule::parse_json(rule).unwrap()).build();
//! // Set up input data
//! let facts = Data { age: 16 };
//! // Evaluate the ruleset on the input data and check if the rule returns
//! // `false`
//! assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![false.into()]);
//! # })
//!  ```
//!
//! Reval can be extended with user-functions by implementing the `UserFunction` trait on a type and passing an instance of that type to the RuleSet. The following example shows how this would work;
//! The input data-type for the rules, must implement serde::Serialize so it
//! can be serialized into a `reval::Value`
//! ```rust
//! # tokio_test::block_on(async {
//! use reval::prelude::*;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Data {
//!     age: u16,
//! }
//! // Set up a FakeId UserFunction that increments an integer `Value` to
//! // bypass the age check
//! struct FakeId;
//! #[async_trait::async_trait]
//! impl UserFunction for FakeId {
//!     async fn call(&self, param: Value) -> anyhow::Result<Value> {
//!         match param {
//!             Value::Int(age) => Ok((age + 5).into()),
//!             _ => Err(anyhow::anyhow!(
//!                 "Invalid value {:?}, expected Value::Int",
//!                 param
//!             )),
//!         }
//!     }
//! }
//! // Set up an "age check" rule that checks if the "age" input field is
//! // greater than or equal to 21. But it first calls the `fake_id` user-
//! // function.
//! let rule = r#"
//! {
//!     "name": "age check",
//!     "expr": {
//!         "gte": [
//!             {"func": ["fake_id", {"ref": "age"}]},
//!             {"int": 21}
//!         ]
//!     }
//! }"#;
//! // Set up the ruleset builder, add the rule, add the user-function and
//! // build the `RuleSet`
//! let ruleset = ruleset()
//!     .with_rule(Rule::parse_json(rule).unwrap())
//!     .with_function("fake_id", FakeId {})
//!     .build();
//! // Set up input data
//! let facts = Data { age: 16 };
//! // Evaluate the ruleset on the input data and check if the rule returns
//! // `true`
//! assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![true.into()]);
//! # })
//!  ```

pub mod error;
pub mod expr;
pub mod function;
#[cfg(feature = "nom_parser")]
pub mod parse;
#[cfg(feature = "json_parser")]
pub mod parse_json;
pub mod ruleset;
pub mod value;

pub use error::{Error, Result};

pub mod prelude {
    pub use crate::function::UserFunction;
    pub use crate::ruleset::{ruleset, Rule, RuleSet};
    pub use crate::value::Value;

    #[cfg(feature = "nom_parser")]
    pub use crate::parse::parse;
}
