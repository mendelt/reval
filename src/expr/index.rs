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
