use crate::{expr::Expr, parse::Error, symbol::Symbols};

impl Symbols {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let expr = Expr::parse(input)?;

        match expr {
            Expr::Map(map) => Ok(Symbols(map)),
            _ => Err(Error::SymbolsFileNotValid),
        }
    }
}

#[cfg(test)]
mod when_parsing_symbols {
    use super::*;
    use crate::value::Value;
    use std::collections::BTreeMap;

    #[test]
    pub fn should_parse_empty_symbols() {
        assert_eq!(Symbols::parse(r#"{}"#), Ok(Symbols(BTreeMap::new())))
    }

    #[test]
    pub fn should_parse_simple_symbol() {
        assert_eq!(
            Symbols::parse(r#"{symbol: i3}"#),
            Ok(Symbols(
                [("symbol".to_string(), Value::Int(3).into())]
                    .into_iter()
                    .collect()
            ))
        )
    }

    #[test]
    pub fn should_parse_multiple_symbol() {
        assert_eq!(
            Symbols::parse(
                r#"{
    symbol_1: i3,
    symbol_2: "some value",
}"#
            ),
            Ok(Symbols(
                [
                    ("symbol_1".to_string(), Value::Int(3).into()),
                    (
                        "symbol_2".to_string(),
                        Value::String("some value".to_string()).into()
                    )
                ]
                .into_iter()
                .collect()
            ))
        )
    }
}
