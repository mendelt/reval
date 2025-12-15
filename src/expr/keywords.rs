use unicode_xid::UnicodeXID;

/// Reserved keywords
const KEYWORDS: [&str; 43] = [
    "and",
    "or",
    "if",
    "then",
    "else",
    "is_some",
    "is_none",
    "some",
    "int",
    "float",
    "dec",
    "true",
    "false",
    "none",
    "contains",
    "in",
    "to_upper",
    "to_lower",
    "uppercase",
    "lowercase",
    "starts",
    "ends",
    "trim",
    "round",
    "floor",
    "fract",
    "date_time",
    "datetime",
    "duration",
    "year",
    "month",
    "week",
    "day",
    "hour",
    "minute",
    "second",
    "any",
    "all",
    "key",
    "val",
    "for",
    "do",
    "filter",
];

pub(crate) fn is_reserved_keyword(name: &str) -> bool {
    KEYWORDS.contains(&name)
}

pub(crate) fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(start) => {
            start == '_' || start.is_xid_start() && chars.all(UnicodeXID::is_xid_continue)
        }
        None => false,
    }
}

#[cfg(test)]
mod when_testing_identifier {
    use crate::expr::keywords::is_valid_identifier;

    #[test]
    fn should_allow_leading_underscore() {
        assert!(is_valid_identifier("_id"));
    }

    #[test]
    fn should_allow_leading_letter() {
        assert!(is_valid_identifier("id"));
    }

    #[test]
    fn should_disallow_leading_number() {
        assert!(!is_valid_identifier("1id"));
    }

    #[test]
    fn should_allow_trailing_number() {
        assert!(is_valid_identifier("id1"));
    }
}
