use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(Number),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Int(i128),
    Float(f64),
}

