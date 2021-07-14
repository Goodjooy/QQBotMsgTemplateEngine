
use std::{str::Chars};

use util::*;


use self::tag::Tag;

pub mod expr;
pub mod iter;
pub mod literal;
pub mod tag;
pub mod util;
pub mod handle;

pub struct PreviewableIter<'a> {
    preview: char,
    offset:usize,
    line:usize,
    iter: Chars<'a>,
}
#[derive(Debug)]
pub enum LexicalType {
    Tag(Tag),
    Literal(literal::Literal)
}

pub struct LexicalHandle<'a>{
    data:PreviewableIter<'a>
}