use crate::anaylze::{syntax::expr::Expression, Value};

use super::{Item, Items};

pub struct Text<'a>(Box<Item<'a>>);

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
    model: CmpMod,

    body: Box<Items<'a>>,
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
    name: String,
    op: ValueOperate,
}

pub enum ValueOperate {
    OutSet,
    New,
    Assign(Expression),
    NewDefault(Expression),
    Print(String),
    Println(String),
}
