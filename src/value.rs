use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Null,
    Number(f32),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
