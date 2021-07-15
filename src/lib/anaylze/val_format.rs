use core::fmt::Display;
use std::collections::vec_deque;

use super::Value;

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Str(s) => s.to_string(),
            Value::List(l) => l
                .iter()
                .map(|f| f.to_string())
                .reduce(|a, b| format!("{}, {}", a, b))
                .and_then(|s| Some(format!("[{}]", s)))
                .unwrap_or("%%Failure To String%%".to_string()),
            Value::UnSet => String::from("Rander Time Set"),
        }
    }
    pub fn format(&self, format: &str) -> String {
        let s = self.to_string();

        format
            .replace("{{", "\0")
            .replace("{}", &s)
            .replace("\0", "{{")
            .replace("{{", "{")
            .replace("}}", "}")
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

#[test]
fn test_format() {
    let v = Value::Int(13);
    let f = v.format("第{}{{}}个{}");

    assert_eq!(f, "第13{}个13");
}

#[test]
fn test_to_string() {
    let v = Value::List(vec![
        Value::Int(11),
        Value::Int(21),
        Value::List(vec![Value::Str("SSS".to_string())]),
    ]);
    assert_eq!(v.to_string(), "[11, 21, [SSS]]")
}
