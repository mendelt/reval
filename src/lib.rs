//! Reval, short for Rust Evaluator is a light-weight expression evaluator library. It can be used as a rules-engine or in other situations where simple user expressions need to be evaluated.
//!
//! Expressions or rules can be written in a simple DSL or in a json format that is directly parsed into Reval expression AST objects. For now the Reval DSL parser is experimental but eventually this should replace json rules.
//!
//! Expressions can be evaluated against input data to produce output data. The input and output data is passed in and out of the rules as a `reval::Value` object which can contain simple data or more complex structures like maps or vectors.
//! Data can even be nested so complex data can be passed into Reval expressions.
//!
//! To make it easy to construct input data from your own datatypes the Reval crate implements a serde serializer for Value types. So any type that implements `serde::Serialize` can be serialized into a `reval::Value` without writing any code.
//!
//! ```rust
//! use reval::{prelude::*, value::ser::ValueSerializer};
//! use serde::Serialize;
//!
//! # tokio_test::block_on(async {
//! let rule =
//!     parse_json(r#"{"name": "age check", "expr": {"gt": [{"ref": "age"}, {"int": 21}]}}"#).unwrap();
//! let mut ruleset = RuleSet::default();
//! ruleset.add_rule(rule);
//!
//! #[derive(Serialize)]
//! struct Data {
//!     age: u16,
//! }
//! let facts = Data { age: 16 }.serialize(ValueSerializer).unwrap();
//!
//! assert_eq!(ruleset.evaluate(&facts).await.unwrap(), vec![false.into()]);
//! # })
//!
//! ```

pub mod error;
pub mod expr;
pub mod function;
#[cfg(feature = "nom_parser")]
pub mod parse;
#[cfg(feature = "json_parser")]
pub mod parse_json;
pub mod ruleset;
pub mod value;

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::ruleset::RuleSet;
    pub use crate::value::Value;

    #[cfg(feature = "nom_parser")]
    pub use crate::parse::parse;

    #[cfg(feature = "json_parser")]
    pub use crate::parse_json::parse_json;
}
