use crate::{expr::Expr, Error, Result};
use std::collections::BTreeMap;

/// Store symbols for use in rules
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Symbols(pub(crate) BTreeMap<String, Expr>);

impl Symbols {
    pub fn insert(&mut self, symbol: impl ToString, value: Expr) {
        self.0.insert(symbol.to_string(), value);
    }

    pub fn append(&mut self, values: impl IntoIterator<Item = (impl ToString, Expr)>) {
        let mut symbols = values
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect();

        self.0.append(&mut symbols)
    }

    pub fn get(&self, symbol: &str) -> Result<&Expr> {
        self.0
            .get(symbol)
            .ok_or(Error::InvalidSymbol(symbol.to_string()))
    }
}

impl<T: IntoIterator<Item = (impl ToString, Expr)>> From<T> for Symbols {
    fn from(symbols: T) -> Self {
        Self(
            symbols
                .into_iter()
                .map(|(name, expr)| (name.to_string(), expr))
                .collect(),
        )
    }
}
