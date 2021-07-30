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
    pub fn into_value(&mut self) -> Option<&mut Value> {
        if let Self::Var(v) = self {
            Some(&mut v.value)
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

    fn add_assi(&mut self, rhs: &Self) -> Option<()> {
        if let Self::Int(i) = self {
            if let Self::Int(ir) = rhs {
                todo!()
            }
        }
        return None;
    }

    fn sub_assi(&mut self, rhs: &Self) -> Option<()> {
        if let Self::Int(i) = self {
            if let Self::Int(ir) = rhs {
                todo!()
            }
        }
        return None;
    }

    fn mul_assi(&mut self, rhs: &Self) -> Option<()> {
        if let Self::Int(i) = self {
            if let Self::Int(ir) = rhs {
                todo!()
            }
        }
        return None;
    }

    fn div_assi(&mut self, rhs: &Self) -> Option<()> {
        if let Self::Int(i) = self {
            if let Self::Int(ir) = rhs {
                todo!()
            }
        }
        return None;
    }
}
