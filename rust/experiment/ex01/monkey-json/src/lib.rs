mod lexer;
mod parser;

use parser::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),                  // 文字列
    Number(f64),                     // 数値
    Bool(bool),                      // 真偽値
    Null,                            // Null
    Array(Vec<Value>),               // JSON Array
    Object(BTreeMap<String, Value>), // JSON Object
}

/// ユーザのエンドポイント
/// 入力のJSON文字列から`Value`を返す
pub fn parse(input: &str) -> Result<Value, ParserError> {
    todo!()
}

/// {"key": true}
/// v["key"] => Value::Bool(true)
impl std::ops::Index<&str> for Value {
    type Output = Value;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Value::Object(map) => map
                .get(key)
                .unwrap_or_else(|| panic!("A key is not found: {}", key)),
            _ => {
                panic!("A value is not object");
            }
        }
    }
}

/// [null, false, 3]
/// v[3] => Value::Number(3f64)
impl std::ops::Index<usize> for Value {
    type Output = Value;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Value::Array(array) => &array[idx],
            _ => {
                panic!("A value is not array");
            }
        }
    }
}
