use crate::anaylze::{syntax::expr::Expression, Value};

use super::Items;

#[derive(Debug)]
pub struct At {
    pub uid: u64,
    pub sep: String,
}
#[derive(Debug)]
pub enum Image {
    Url(String),
    File(String),
    Base64(String),
}

pub struct Sign {
    pub sign: String,
    pub repeat: u32,
}

pub struct If {
    pub model: CmpMod,
    pub body: Box<Items>,
    pub follows: IfFollows,
}

pub enum IfFollows {
    Nil,
    Elif(Box<If>),
    Else(Box<Items>),
}

pub struct Loop {
    pub times: Expression,
    pub name: Option<String>,

    pub body: Box<Items>,
}

pub struct For {
    pub source: Value,
    pub name: String,

    pub body: Box<Items>,
}

pub struct While {
    pub model: CmpMod,

    pub body: Box<Items>,
}

pub enum CmpMod {
    Eq(Expression, Expression),
    Neq(Expression, Expression),

    Gt(Expression, Expression),
    Gte(Expression, Expression),

    Lt(Expression, Expression),
    Lte(Expression, Expression),

    BoolT(Expression),
    BoolF(Expression),
}


pub struct Var {
    pub name: String,
    pub op: ValueOperate,
}

pub enum ValueOperate {
    // need user prvide this value
    // into render
    Need,
    // create new value on sign tabel 
    //but with value Unset
    New,
    // assign a value in sign tabel
    // if not exist create it 
    Assign(Expression),
    // format the value into text
    Print(String),
    // format the value into text with 
    // \n at the end
    Println(String),
}
