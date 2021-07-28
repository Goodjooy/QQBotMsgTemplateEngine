use crate::lib::anaylze::lexical::expr::ExprIter;
use crate::lib::anaylze::{SignTableHandle};
use crate::lib::anaylze::syntax::SyntaxLoadNext;
use crate::lib::anaylze::Var;

use self::structs::{At, For, If, Image, Loop, Sign, Text, While};

//tag分析符号
mod structs;

mod loops;
mod ulit;

pub struct Items<'a>(ItemMeta<'a>,Item<'a>);

/// * 语法分析单元
/// * 
pub enum   Item<'a> {
    Item(ItemMeta<'a>, Box<Item<'a>>),
    Nil
}

pub enum ItemMeta<'a> {
    Literal(Literal),
    Tag(Tag<'a>),
}
// 字面量分析单元，对应任何不在标签内部的文本
pub struct Literal (String);

// tag分析单元，分析全部tag类型
pub enum Tag<'a> {
    Ctrl(CtrlTag<'a>),
    Info(InfoTag<'a>),
    Liter(LiteralTag)
    
}
// 控制tag 控制流tag
pub enum CtrlTag<'a> {
    If(If<'a>),
    Loops(Loops<'a>),
    Var(Var),
}
// 循环
pub enum Loops<'a> {
    For(For<'a>),
    Loop(Loop<'a>),
    While(While<'a>),
}
// 消息显示tag
pub enum InfoTag<'a> {
    Img(Image),
    At(At<'a>),

}

// 显示字面量的tag
pub enum LiteralTag {
    Sign(Sign),
    Endl
}

