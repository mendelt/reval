use std::collections::hash_map::HashMap;

pub enum Value {
    String(String),
    Number(Number),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}

pub enum Number {
    UInt(u128),
    Int(i128),
    Float(f64),
}

