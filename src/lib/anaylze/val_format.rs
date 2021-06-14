use super::Value;

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Uint(u) => u.to_string(),
            Value::F32(f) => f.to_string(),
            Value::F64(f) => f.to_string(),
            Value::Str(s) => s.to_string(),
        }
    }
    fn format(&self, format: &str) -> String {
        let s = self.to_string();

        format
            .replace("{{", "\0")
            .replace("{}", &s)
            .replace("\0", "{{")
            .replace("{{", "{")
            .replace("}}", "}")
    }
}

#[test]
fn test_format() {
    let v = Value::Int(13);
    let f = v.format("第{}{{}}个{}");

    assert_eq!(f, "第13{}个13");
    assert_eq!(Value::F32(1.11).format("-{}-"),"-1.11-");
}

