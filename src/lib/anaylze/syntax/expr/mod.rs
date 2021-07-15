use crate::lib::anaylze::Value;
use crate::lib::anaylze::SignTableHandle;
use crate::lib::anaylze::Sign;
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
pub struct ExprVar<'a>(&'a Var);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Factor<'a> {
    Digit(i64),
    SubExpr(Box<Expression<'a>>),
    Var(ExprVar<'a>),
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SubItem<'a> {
    Multiple(Factor<'a>, Box<SubItem<'a>>),
    Division(Factor<'a>, Box<SubItem<'a>>),
    Nil,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Item<'a>(Factor<'a>, SubItem<'a>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SubCaculate<'a> {
    Addition(Item<'a>, Box<SubCaculate<'a>>),
    Subtraction(Item<'a>, Box<SubCaculate<'a>>),
    Nil,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Caculate<'a>(Item<'a>, SubCaculate<'a>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression<'a> {
    Caculate(Caculate<'a>),
    Literal(Literal),
}

pub fn nil_sign<'a, T,N>(err: LoadErr, nil: T) -> Result<LoadStatus< T,N>, LoadErr> {
    println!("{:?}",err);
    match err {
        LoadErr::IterEnd => Ok(LoadStatus::ok(nil)),
        LoadErr::UnexprectLetical(_)|LoadErr::UnSupportOperate(_) => Err(err),
        
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
}

impl LexIter {
    fn new()->Self{
        LexIter{
            d:Sign::Var(Var{name:"".to_string(),value:Value::Int(-11)}),
            u:Sign::Var(Var{name:"".to_string(),value:Value::Int(11)}),
            s:Sign::Var(Var{name:"".to_string(),value:Value::Str("SSSS".to_string())}),

        }
    }
}

#[macro_export]
macro_rules! test_data {
    ($x:expr) => {
        let mut signs=LexIter::new();
        let iter=PreviewableIter::new(stringify!(x));
        let mut expr=ExprIter::new(&mut signs, iter);
    };
}