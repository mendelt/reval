use std::num::ParseIntError;
use thiserror::Error;

pub fn unescape(value: &str) -> Result<String, UnescapeError> {
    let mut chars = value.chars().enumerate();
    let mut res = String::with_capacity(value.len());

    while let Some((idx, c)) = chars.next() {
        res.push(if c == '\\' {
            match chars.next() {
                None => Err(UnescapeError::InvalidEscape {
                    escape: format!("{}", c),
                    index: idx,
                    string: String::from(value),
                }),
                Some((idx, c2)) => match c2 {
                    'n' => Ok('\n'),
                    'r' => Ok('\r'),
                    't' => Ok('\t'),
                    '\\' => Ok('\\'),
                    '\'' => Ok('\''),
                    '"' => Ok('"'),
                    'u' => parse_unicode(&mut chars).map_err(|x| UnescapeError::InvalidUnicode {
                        source: x,
                        index: idx,
                        string: String::from(value),
                    }),
                    _ => Err(UnescapeError::InvalidEscape {
                        escape: format!("{}{}", c, c2),
                        index: idx,
                        string: String::from(value),
                    }),
                },
            }
        } else {
            Ok(c)
        }?);
    }

    Ok(res)
}

fn parse_unicode<I>(chars: &mut I) -> Result<char, ParseUnicodeError>
where
    I: Iterator<Item = (usize, char)>,
{
    match chars.next() {
        Some((_, '{')) => {}
        _ => {
            return Err(ParseUnicodeError::BraceNotFound);
        }
    }

    let unicode_seq: String = chars
        .take_while(|&(_, c)| c != '}')
        .map(|(_, c)| c)
        .collect();

    u32::from_str_radix(&unicode_seq, 16)
        .map_err(|e| ParseUnicodeError::ParseHexFailed {
            source: e,
            string: unicode_seq,
        })
        .and_then(|u| char::from_u32(u).ok_or(ParseUnicodeError::ParseUnicodeFailed { value: u }))
}

/// Error type of [unescape](unescape).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UnescapeError {
    #[error("invalid escape {escape} at {index} in {string}")]
    InvalidEscape {
        escape: String,
        index: usize,
        string: String,
    },
    #[error("\\u could not be parsed at {index} in {string}: {source}")]
    InvalidUnicode {
        #[source]
        source: ParseUnicodeError,
        index: usize,
        string: String,
    },
}

/// Source error type of [UnescapeError::InvalidUnicode](UnescapeError::InvalidUnicode).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseUnicodeError {
    #[error("expected '{{' character in unicode escape")]
    BraceNotFound,
    #[error("could not parse {string} as u32 hex: {source}")]
    ParseHexFailed {
        #[source]
        source: ParseIntError,
        string: String,
    },
    #[error("could not parse {value} as a unicode char")]
    ParseUnicodeFailed { value: u32 },
}
