use crate::lib::anaylze::syntax::expr::Expression;


use crate::lib::anaylze::Value;

use super::Item;

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
    sign: String,
    repeat: u32,
}

pub struct If<'a> {
    model: CmpMod<'a>,
    body: Box<Item<'a>>,
}

pub struct Loop<'a> {
    times: u32,
    name: String,

    body: Box<Item<'a>>,
}

pub struct For<'a> {
    source: &'a Value,
    name: String,

    body: Box<Item<'a>>,
}

pub struct While<'a> {
    model: CmpMod<'a>,

    body: Box<Item<'a>>,
}

pub enum CmpMod<'a> {
    Eq(&'a Value, &'a Value),
    Neq(&'a Value, &'a Value),

    Gt(&'a Value, &'a Value),
    Gte(&'a Value, &'a Value),

    Lt(&'a Value, &'a Value),
    Lte(&'a Value, &'a Value),

    BoolT(&'a Value),
    BoolF(&'a Value),
}

pub struct Var<'a> {
    name:String,
    op:ValueOperate<'a>
}

pub enum ValueOperate<'a> {
    Assign(Expression<'a>),
    New,
    NewDefault(Expression<'a>),
    Print(String),
    Println(String),
}
