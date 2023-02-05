use unicode_xid::UnicodeXID;

const IF: &str = "if";
const REF: &str = "ref";
const NEG: &str = "neg";
const FLOAT: &str = "float";
const INT: &str = "int";
const DEC: &str = "dec";

const KEYWORDS: [&'static str; 6] = [IF, REF, NEG, FLOAT, INT, DEC];

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
mod test {}
