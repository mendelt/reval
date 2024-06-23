//! Reval, short for Rust Evaluator is a light-weight expression evaluator library. It can be used as a rules-engine or in other situations where simple user expressions need to be evaluated.
//!
//! Expressions or rules can be written in a simple DSL or in a json format that is directly parsed into Reval expression AST objects. For now the Reval DSL parser is experimental but eventually this should replace json rules.
//!
//! Expressions can be evaluated against input data to produce output data. The input and output data is passed in and out of the rules as a `reval::Value` object which can contain simple data or more complex structures like maps or vectors.
//! Data can be nested so complex data can be passed into Reval expressions.
//!
//! To make it easy to construct input data from your own datatypes the Reval crate implements a serde serializer for Value types. So any type that implements `serde::Serialize` can be serialized into a `reval::Value` without writing any code.
//!
//! More information on writing rules can be found here: [Writing Rules](https://mendelt.github.io/reval/)
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
//!
//! // Set up an "age check" rule that checks if the "age" input field is
//! // greater than or equal to 21
//! let rule = r"
//! // age check
//! age >= i21
//! ";
//!
//! // Set up the ruleset builder, add the rule and build the `RuleSet`
//! let ruleset = ruleset().with_rule(Rule::parse(rule).unwrap()).unwrap().build();
//! // Set up input data
//! let facts = Data { age: 16 };
//! // Evaluate the ruleset on the input data and check if the rule returns
//! // `false`
//! for outcome in ruleset.evaluate(&facts).await.unwrap() {
//!     assert_eq!(outcome.value.unwrap(), false.into());
//! }
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
//!
//! // Set up a FakeId UserFunction that increments an integer `Value` to
//! // bypass the age check
//! struct FakeId;
//! #[async_trait::async_trait]
//! impl UserFunction for FakeId {
//!     async fn call(&self, param: Value) -> FunctionResult {
//!        let age: i128 = param.try_into()?;
//!        Ok((age * 2).into())
//!     }
//!
//!     fn name(&self) -> &'static str {
//!         "fake_id"
//!     }
//! }
//!
//! // Set up an "age check" rule that checks if the "age" input field is
//! // greater than or equal to 21. But it first calls the `fake_id` user-
//! // function.
//! let rule = r"
//! // age check
//! fake_id(age) >= i21
//! ";
//!
//! // Set up the ruleset builder, add the rule, add the user-function and
//! // build the `RuleSet`
//! let ruleset = ruleset()
//!     .with_rule(Rule::parse(rule).unwrap()).unwrap()
//!     .with_function(FakeId {}).unwrap()
//!     .build();
//! // Set up input data
//! let facts = Data { age: 16 };
//! // Evaluate the ruleset on the input data and check if the rule returns
//! // `true`
//! for outcome in ruleset.evaluate(&facts).await.unwrap() {
//!     assert_eq!(outcome.value.unwrap(), true.into());
//! }
//! # })
//!  ```

pub mod error;
pub mod expr;
pub mod function;
pub mod parse;
pub mod ruleset;
pub mod symbol;
pub mod value;

pub use error::{Error, Result};

pub mod prelude {
    pub use crate::{
        expr::Expr,
        function::{FunctionResult, UserFunction},
        ruleset::{ruleset, Builder, Rule, RuleSet},
        value::Value,
    };
}
