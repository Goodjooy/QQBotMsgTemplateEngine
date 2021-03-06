use crate::anaylze::lexical::expr::ExprIter;

use self::structs::{At, For, If, Image, Loop, Sign, Var, While};

//tag分析符号
mod cmp;
mod structs;

mod items;

mod lextical;

mod tag;
mod ctrl_tag;
mod info_tag;
mod literal_tag;

mod util;

pub trait TagInfo {
    fn tag_name() -> &'static str;
    fn accept_full() -> bool {
        false
    }
}

pub struct Items(ItemMeta, Item);

/// * 语法分析单元
/// *
pub enum Item {
    Item(ItemMeta, Box<Item>),
    Nil,
}

pub enum ItemMeta {
    // 字面量分析单元，对应任何不在标签内部的文本
    Lit(String),
    Tag(Tag),
}

// tag分析单元，分析全部tag类型
pub enum Tag {
    Ctrl(CtrlTag),
    Info(InfoTag),
    Liter(LiteralTag),
}
// 控制tag 控制流tag
pub enum CtrlTag {
    If(If),
    Loops(Loops),
    Var(Var),
}
// 循环
pub enum Loops {
    For(For),
    Loop(Loop),
    While(While),
}
// 消息显示tag
pub enum InfoTag {
    Img(Image),
    At(At),
}

// 显示字面量的tag
pub enum LiteralTag {
    Sign(Sign),
    Endl,
}
