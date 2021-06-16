use crate::lib::anaylze::{Value, Var};

pub struct Literal(String);
pub struct ExprVar(Var);

pub enum Factor {
    Digit(i64),
    SubExpr(),
    Var(ExprVar),
}

pub enum SubItem {
    Multiple(Factor, Box<SubItem>),
    Division(Factor, Box<SubItem>),
    Nil,
}
pub struct Item(Factor, SubItem);

pub enum SubCaculate {
    Addition(Item, Box<SubCaculate>),
    Subtraction(Item, Box<SubCaculate>),
    Nil,
}

pub struct Caculate(Item, SubCaculate);

pub enum Expression {
    Caculate(Caculate),
    Literal(Literal),
    Value(ExprVar),
}
