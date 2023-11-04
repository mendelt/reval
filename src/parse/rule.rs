use crate::{expr::Expr, parse::Error, ruleset::rule::Rule};
use itertools::Itertools;

impl Rule {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut comment_lines = input
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix("//").map(str::trim));

        match comment_lines.next() {
            Some(name_line) => {
                let name = name_line.trim();
                let description = comment_lines.map(|line| line.trim()).join("\n");

                Ok(Rule::new(name, description, Expr::parse(input)?))
            }
            None => Err(Error::MissingRuleName),
        }
    }
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
    fn should_error_when_no_expression() {
        assert!(Rule::parse("//name\n//descr1\n").is_err());
    }

    #[test]
    fn should_error_when_expression_invalid() {
        assert!(Rule::parse("//name\n34").is_err(),);
    }

    #[test]
    fn should_parse_multiline_expression() {
        assert_eq!(
            Rule::parse("//name\n// descr1\n// descr2  \ni3")
                .unwrap()
                .description(),
            Some("descr1\ndescr2")
        );
    }

    #[test]
    fn should_parse_comments_in_between_expression() {
        assert_eq!(
            Rule::parse("//name\n// descr1  \t\ni3\n// descr2")
                .unwrap()
                .description(),
            Some("descr1\ndescr2")
        );
    }
}
