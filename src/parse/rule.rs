use super::Parsers;
use crate::{ruleset::rule::Rule, Error};

impl Parsers for Rule {
    type Error = Error;
    type Parseable = Rule;

    fn parse(_input: &str) -> Result<Rule, Error> {
        todo!();
    }
}

#[cfg(test)]
mod when_parsing_rules {
    // use super::*;

    #[test]
    #[ignore]
    fn should_error_when_first_line_not_rule_name() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_parse_comments_as_decription() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_trim_rule_name() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_trim_rule_name_spaces() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_error_when_no_expression() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_parse_multiline_expression() {
        todo!()
    }

    #[test]
    #[ignore]
    fn should_parse_comments_in_between_expression() {
        todo!()
    }
}
