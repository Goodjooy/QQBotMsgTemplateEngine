use core::str;
use std::{cmp::PartialEq, fmt::Display, str::Chars};

use util::*;


use self::tag::Tag;

use super::Sign;

mod expr;
mod iter;
mod literal;
mod tag;
mod util;
mod handle;

pub struct PreviewableIter<'a> {
    preview: char,
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