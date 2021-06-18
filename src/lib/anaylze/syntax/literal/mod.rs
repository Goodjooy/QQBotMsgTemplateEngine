use crate::lib::anaylze::Var;

use self::structs::{For, If, Image, Loop, Sign, Text, While};

//tag分析符号
mod structs;

struct Item<'a> (ItemMeta<'a>,Box<Item<'a>>);

enum ItemMeta<'a> {
    Literal(Literal<'a>),
    Tag(Tag<'a>)
}

enum Literal<'a> {
    Normal(String),
    Tag(LiteralTag<'a>)
}
enum LiteralTag<'a> {
    Text(Text<'a>),
    Sign(Sign),
    Endl
}

enum Tag<'a> {
    If(If<'a>),
    Loops(Loops<'a>),
    Var(Var),
    Img(Image),

}
enum Loops<'a> {
    For(For<'a>),
    Loop(Loop<'a>),
    While(While<'a>),
}