//! User functions
mod context;
mod user_functions;

use crate::value::Value;
use async_trait::async_trait;
pub use context::FunctionContext;
use std::result;
pub use user_functions::UserFunctions;

/// User functions should implement this trait
#[async_trait]
pub trait UserFunction {
    /// Call the userfunction, parameters are passed in as a Value
    async fn call(&self, params: Value) -> FunctionResult;

    /// The name of the user-function
    fn name(&self) -> &'static str;

    /// Indicates if results of this function can be cached,
    /// true by default
    fn cacheable(&self) -> bool {
        true
    }
}

/// Convenience type for passing around boxed user-function implementations
pub type BoxedFunction = Box<dyn UserFunction + Send + Sync + 'static>;

/// Result type returned from UserFunction
pub type FunctionResult = result::Result<Value, anyhow::Error>;
