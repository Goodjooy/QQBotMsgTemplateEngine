use super::{Sign, Value, Var};
use std::ops::{Add, Div, Mul, Sub};

impl Var {
    pub fn new(name: &str, value: Value) -> Self {
        Self {
            name: name.to_string(),
            value: value,
        }
    }
}

impl Sign {
    pub fn is_value(&self) -> bool {
        if let Self::Var(_) = self {
            true
        } else {
            false
        }
    }
    pub fn into_value_mut(&mut self) -> Option<&mut Value> {
        if let Self::Var(v) = self {
            Some(&mut v.value)
        } else {
            None
        }
    }
    pub fn into_value(&self) -> Option<& Value> {
        if let Self::Var(v) = self {
            Some(& v.value)
        } else {
            None
        }
    }
}

impl Value {
    pub fn into_var(self, name: &str) -> Var {
        Var {
            name: name.to_string(),
            value: self,
        }
    }
}

impl Value {
    pub fn set_as(&mut self, target: Self) -> Option<()> {
        match self {
            Value::UnSet => *self = target,
            Value::Int(_) => {
                if let Value::Int(is) = target {
                    *self = Value::Int(is)
                } else {
                    return None;
                }
            }
            Value::Str(_) => {
                if let Value::Str(is) = target {
                    *self = Value::Str(is)
                } else {
                    return None;
                }
            }
            Value::List(_) => {
                if let Value::List(is) = target {
                    *self = Value::List(is)
                } else {
                    return None;
                }
            }
        }

        Some(())
    }

    pub fn add_assgin(&mut self, rhs: i64) -> Option<()> {
        if let Self::Int(i) = self {
            *i += rhs;
            Some(())
        } else {
            None
        }
    }

    pub fn sub_assgin(&mut self, rhs: i64) -> Option<()> {
        if let Self::Int(i) = self {
            *i -= rhs;
            Some(())
        } else {
            None
        }
    }

    pub fn mul_assgin(&mut self, rhs: i64) -> Option<()> {
        if let Self::Int(i) = self {
            *i *= rhs;
            Some(())
        } else {
            None
        }
    }

    pub fn div_assign(&mut self, rhs: i64) -> Option<()> {
        if let Self::Int(i) = self {
            *i /= rhs;
            Some(())
        } else {
            None
        }
    }

    pub fn push(&mut self, data: Value) -> Option<()> {
        if let Self::List(l) = self {
            l.push(data);
            Some(())
        } else if let Self::Str(s) = self {
            if let Value::Str(sd) = data {
                s.push_str(&sd);
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}
