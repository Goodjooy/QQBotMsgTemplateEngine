use crate::anaylze::{syntax::expr::Expression, Value};

use super::{Item, Items};


pub struct At<'a> {
    uid: u64,
    sep: &'a str,
}
pub enum Image {
    URL(String),
    File(String),
}

pub struct Sign {
    pub sign: String,
    pub repeat: u32,
}

pub struct If<'a> {
    pub model: CmpMod,
    pub body: Box<Items<'a>>,
    pub follows: IfFollows<'a>,
}

pub enum IfFollows<'a> {
    Nil,
    Elif(Box<If<'a>>),
    Else(Box<Items<'a>>),
}

pub struct Loop<'a> {
    pub times: Expression,
    pub name: Option<String>,

    pub body: Box<Items<'a>>,
}

pub struct For<'a> {
    pub source: Value,
    pub name: String,

    pub body: Box<Items<'a>>,
}

pub struct While<'a> {
    pub model: CmpMod,

    pub body: Box<Items<'a>>,
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

pub struct Need{
    pub name:String
}

pub struct Var {
    pub name: String,
    pub op: ValueOperate,
}

pub enum ValueOperate {
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
