use crate::{
    expr::Expr,
    parse::{helpers::RevalParseError, reval, Error},
    ruleset::Rule,
    value::Value,
};
use std::collections::BTreeMap;

impl Rule {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut comment_lines = input
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix("//").map(str::trim));

        let mut rule_builder = reval::RuleParser::new()
            .parse(input)
            .map_err(|error| Error::RuleParseError(error.to_string()))?;

        let (name, description) = match comment_lines.next() {
            Some(name_line) => {
                let name = name_line;

                // let description = comment_lines.map(str::trim).join("\n");
                let desc_first = comment_lines.next();
                let description = desc_first.map(|first| {
                    comment_lines.fold(first.to_string(), |acc, next| format!("{acc}\n{next}"))
                });

                (Some(name), description)
            }
            None => (None, None),
        };

        if let Some(name) = name {
            rule_builder = rule_builder.set_name(name);
        }

        if let Some(description) = description {
            rule_builder = rule_builder.set_description(&description);
        }

        rule_builder.build()
    }
}

const DESCRIPTION_META: &str = "description";
const NAME_META: &str = "name";

/// Build rules from parsed components
pub(crate) struct RuleBuilder {
    name: Option<String>,
    expr: Expr,
    metadata: BTreeMap<String, Value>,
}

impl RuleBuilder {
    pub(crate) fn parse(meta: Vec<(String, Expr)>, expr: Expr) -> Result<Self, RevalParseError> {
        let mut metadata: BTreeMap<String, Value> = BTreeMap::new();
        let mut name = None;

        for (key, expr) in meta {
            match (&key[..], flatten(expr)) {
                (NAME_META, Ok(Value::String(name_value))) => {
                    name = Some(name_value);
                }
                (NAME_META, Ok(_)) => {
                    return Err(RevalParseError::InvalidNameValue);
                }
                (_, Ok(value)) => {
                    metadata.insert(key, value);
                }
                (_, Err(_)) => {
                    return Err(RevalParseError::InvalidMetadata(key));
                }
            }
        }

        Ok(Self {
            name,
            expr,
            metadata,
        })
    }

    /// Set the name if it wasnt already set from metadata
    pub(crate) fn set_name(mut self, name: &str) -> Self {
        if self.name.is_none() {
            self.name = Some(name.to_string())
        }

        self
    }

    pub(crate) fn set_description(mut self, description: &str) -> Self {
        if !self.metadata.contains_key(DESCRIPTION_META) {
            self.metadata
                .insert(DESCRIPTION_META.to_string(), description.into());
        };

        self
    }

    pub(crate) fn build(self) -> Result<Rule, Error> {
        Ok(Rule::new(
            self.name.ok_or(Error::MissingRuleName)?,
            self.metadata,
            self.expr,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
enum FlattenError {
    #[error("Invalid metadata expression")]
    InvalidMetadata,
}

fn flatten(expr: Expr) -> Result<Value, FlattenError> {
    match expr {
        Expr::Value(value) => Ok(value),
        Expr::Vec(values) => Ok(Value::Vec(
            values
                .into_iter()
                .map(flatten)
                .collect::<Result<Vec<Value>, FlattenError>>()?,
        )),
        Expr::Map(values) => Ok(Value::Map(
            values
                .into_iter()
                .map(flatten_keyvalue)
                .collect::<Result<BTreeMap<String, Value>, FlattenError>>()?,
        )),
        _ => Err(FlattenError::InvalidMetadata),
    }
}

fn flatten_keyvalue((key, expr): (String, Expr)) -> Result<(String, Value), FlattenError> {
    Ok((key, flatten(expr)?))
}

#[cfg(test)]
mod when_parsing_rules {
    use super::*;

    #[test]
    fn should_error_when_rule_name_missing() {
        assert!(matches!(Rule::parse("i3"), Err(Error::MissingRuleName)));
    }

    #[test]
    fn should_parse_rule_name() {
        assert_eq!(Rule::parse("//rule name\ni3").unwrap().name(), "rule name");
    }

    #[test]
    fn should_trim_rule_name() {
        assert_eq!(
            Rule::parse("// valid rule name   \t\ni3").unwrap().name(),
            "valid rule name"
        );
    }

    #[test]
    fn should_parse_comments_as_description() {
        assert_eq!(
            Rule::parse("//name\n//descr1\ni3").unwrap().description(),
            Some("descr1")
        );
    }

    #[test]
    fn should_return_none_when_no_description_specified() {
        assert_eq!(Rule::parse("//rule name\ni3").unwrap().description(), None);
    }

    #[test]
    fn should_error_when_no_expression() {
        assert!(Rule::parse("//name\n//descr1\n").is_err());
    }

    #[test]
    fn should_error_when_expression_invalid() {
        assert!(Rule::parse("//name\n34").is_err(),);
    }

    #[test]
    fn should_parse_multiline_description() {
        assert_eq!(
            Rule::parse(
                r#"
//name
// descr1
// descr2  
i3"#
            )
            .unwrap()
            .description(),
            Some("descr1\ndescr2")
        );
    }

    #[test]
    fn should_parse_comments_in_between_expression() {
        assert_eq!(
            Rule::parse(
                r#"
//name
// descr1  
i3
// descr2"#
            )
            .unwrap()
            .description(),
            Some("descr1\ndescr2")
        );
    }

    #[test]
    fn should_parse_metadata_field() {
        assert_eq!(
            Rule::parse(
                r#"
//name
@meta: "Meta field 1";
i3"#
            )
            .unwrap()
            .get_metadata("meta"),
            Some(&Value::from("Meta field 1"))
        );
    }

    #[test]
    fn should_parse_multiple_metadata_fields() {
        let rule = Rule::parse(
            r#"
//name
@meta1: "Meta field 1";
@meta2: "Meta field 2";
i3"#,
        )
        .unwrap();

        assert_eq!(
            rule.get_metadata("meta1"),
            Some(&Value::from("Meta field 1"))
        );

        assert_eq!(
            rule.get_metadata("meta2"),
            Some(&Value::from("Meta field 2"))
        );
    }

    #[test]
    fn should_parse_metadata_case_sensitive() {
        let rule = Rule::parse(
            r#"
//name
@meta: "Meta field lower";
@Meta: "Meta field capitalized";
i3"#,
        )
        .unwrap();

        assert_eq!(
            rule.get_metadata("meta"),
            Some(&Value::from("Meta field lower"))
        );

        assert_eq!(
            rule.get_metadata("Meta"),
            Some(&Value::from("Meta field capitalized"))
        );
    }

    #[test]
    fn should_parse_metadata_int() {
        let rule = Rule::parse(
            r#"
//name
@meta: i15;
i3"#,
        )
        .unwrap();

        assert_eq!(rule.get_metadata("meta"), Some(&Value::from(15)));
    }

    #[test]
    fn should_parse_metadata_float() {
        let rule = Rule::parse(
            r#"
//name
@meta: f1.2;
i3"#,
        )
        .unwrap();

        assert_eq!(rule.get_metadata("meta"), Some(&Value::from(1.2f64)));
    }

    #[test]
    fn should_parse_metadata_vec() {
        let rule = Rule::parse(
            r#"
//name
@meta: ["meta1", "meta2"];
i3"#,
        )
        .unwrap();

        assert_eq!(
            rule.get_metadata("meta").unwrap(),
            &Value::Vec(vec!["meta1".into(), "meta2".into(),].into_iter().collect())
        );
    }

    #[test]
    fn should_parse_metadata_map() {
        let rule = Rule::parse(
            r#"
//name
@meta: {meta1: i1, meta2: i2};
i3"#,
        )
        .unwrap();

        assert_eq!(
            rule.get_metadata("meta").unwrap(),
            &Value::Map(
                [
                    ("meta1".to_string(), 1.into()),
                    ("meta2".to_string(), 2.into())
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn should_parse_name_from_metadata() {
        let rule = Rule::parse(
            r#"
@name: "rule name";
i3"#,
        )
        .unwrap();

        assert_eq!(rule.name(), "rule name");
    }

    #[test]
    fn should_parse_description_from_metadata() {
        let rule = Rule::parse(
            r#"
@name: "rule name";
@description: "some rule description";
i3"#,
        )
        .unwrap();

        assert_eq!(rule.description(), Some("some rule description"));
    }

    #[test]
    fn should_overwrite_rulename_with_metadata() {
        let rule = Rule::parse(
            r#"
//name
@name: "new name";
i3"#,
        )
        .unwrap();

        assert_eq!(rule.name(), "new name");
    }

    #[test]
    fn should_overwrite_rule_description_with_metadata() {
        let rule = Rule::parse(
            r#"
//name
//description
@description: "new description";
i3"#,
        )
        .unwrap();

        assert_eq!(rule.description(), Some("new description"));
    }
}
