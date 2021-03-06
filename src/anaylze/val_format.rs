use crate::anaylze::Sign;
use core::fmt::Display;

use super::{Value, Var};

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
            Value::UnSet(s) => String::from("Rander Time Set"),
            Value::Bool(b) => b.to_string(),
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
impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Var(var) => write!(f, "Value: {}", &var),
            Sign::Const(v) => write!(f, "Const: {}", &v),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} | value: {}", self.name, self.value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test{
    use super::*;
    
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
        
    }