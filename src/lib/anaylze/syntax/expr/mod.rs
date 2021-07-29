use crate::lib::anaylze::Sign;
use crate::lib::anaylze::SignTableHandle;
use crate::lib::anaylze::Value;
use crate::lib::anaylze::Var;

use super::{LoadErr, LoadStatus};

mod caculate;
mod factor;
mod handle;
mod item;
mod literal;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Literal(String);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExprVar(Var);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Factor {
    Digit(i64),
    SubExpr(Box<Expression>),
    Var(ExprVar),
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SubItem {
    Multiple(Factor, Box<SubItem>),
    Division(Factor, Box<SubItem>),
    Nil,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Item(Factor, SubItem);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SubCaculate {
    Addition(Item, Box<SubCaculate>),
    Subtraction(Item, Box<SubCaculate>),
    Nil,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Caculate(Item, SubCaculate);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Caculate(Caculate),
    Literal(Literal),
}

pub fn nil_sign<'a, T, N>(err: LoadErr, nil: T) -> Result<LoadStatus<T, N>, LoadErr> {
    println!("{:?}", err);
    match err {
        LoadErr::IterEnd => Ok(LoadStatus::ok(nil)),
        LoadErr::UnexprectLetical(_)
        | LoadErr::UnSupportOperate(_)
        | LoadErr::TargetAttrNotExist(_) => Err(err),
    }
}

///test struct
struct LexIter {
    d: Sign,
    u: Sign,
    s: Sign,
}

impl SignTableHandle for LexIter {
    fn check_exist(&self, key: &str) -> bool {
        match key {
            "test_D" | "test_U" | "test_S" => true,
            _ => false,
        }
    }

    fn get_sign(&self, key: &str) -> Option<&crate::lib::anaylze::Sign> {
        match key {
            "test_D" => Some(&self.d),
            "test_U" => Some(&self.u),
            "test_S" => Some(&self.s),
            _ => None,
        }
    }

    fn get_mut_sign(&mut self, key: &str) -> Option<&mut crate::lib::anaylze::Sign> {
        match key {
            "test_D" => Some(&mut self.d),
            "test_U" => Some(&mut self.u),
            "test_S" => Some(&mut self.s),
            _ => None,
        }
    }

    fn new_sign(&mut self, _key: &str, _value: crate::lib::anaylze::Sign) -> Option<()> {
        None
    }

    fn leave(&mut self) {
        todo!()
    }

    fn enter(&mut self) {
        todo!()
    }
}

impl LexIter {
    fn new() -> Self {
        LexIter {
            d: Sign::Var(Var {
                name: "".to_string(),
                value: Value::Int(-11),
            }),
            u: Sign::Var(Var {
                name: "".to_string(),
                value: Value::Int(11),
            }),
            s: Sign::Var(Var {
                name: "".to_string(),
                value: Value::Str("SSSS".to_string()),
            }),
        }
    }
}

#[macro_export]
macro_rules! test_data {
    ($x:expr) => {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new(stringify!(x));
        let mut expr = ExprIter::new(&mut signs, iter);
    };
}
