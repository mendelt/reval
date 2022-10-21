use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}
