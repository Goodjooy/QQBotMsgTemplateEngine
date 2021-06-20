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
    Value(ExprVar<'a>),
}

pub fn nil_sign<'a, T>(err: LoadErr, nil: T) -> Result<LoadStatus<'a, T>, LoadErr> {
    match err {
        LoadErr::IterEnd => Ok(LoadStatus::ok(nil)),
        LoadErr::UnexprectLetical(s) => Err(LoadErr::UnexprectLetical(s)),
    }
}
