use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Index {
    Map(String),
    Vec(usize),
}

impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Index::Vec(value)
    }
}

impl From<&str> for Index {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Index {
    fn from(value: String) -> Self {
        Index::Map(value)
    }
}

impl Display for Index {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Map(index) => write!(formatter, "{index}"),
            Index::Vec(index) => write!(formatter, "{index}"),
        }
    }
}
