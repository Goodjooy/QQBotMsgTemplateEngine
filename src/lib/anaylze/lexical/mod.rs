use std::str::Chars;

use util::*;

use self::tag::Tag;

use super::{PreviewIter, SignTableHandle};

pub mod expr;
pub mod handle;
pub mod iter;
pub mod literal;
pub mod tag;
pub mod util;
pub struct PreviewableIter<'a> {
    preview: char,
    offset: usize,
    line: usize,
    iter: Chars<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum LexicalType {
    Tag(Tag),
    Literal(literal::Literal),
    Nil,
}
pub struct LexicalHandle<'a> {
    data: PreviewableIter<'a>,
}

pub struct OutDataLoader<'a, S>(LexicalHandle<'a>, & 'a mut S, LexicalType)
where
    S: SignTableHandle;

impl<'a, S: SignTableHandle> Iterator for OutDataLoader<'a, S> {
    type Item = LexicalType;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.2.clone();
        self.2 = self.0.next().or(Some(LexicalType::Nil))?;

        if let LexicalType::Nil = temp {
            None
        } else {
            Some(temp)
        }
    }
}
impl<'a, S: SignTableHandle> PreviewIter for OutDataLoader<'a, S> {
    fn preview(&self) -> Option<Self::Item> {
        let temp = self.2.clone();
        if let LexicalType::Nil = temp {
            None
        } else {
            Some(temp)
        }
    }
}

impl<'a, S: SignTableHandle> OutDataLoader<'a, S> {
    pub fn new(signs: & 'a mut S, iter: PreviewableIter<'a>) -> Self {
        let mut t = OutDataLoader(LexicalHandle { data: iter }, signs, LexicalType::Nil);
        t.next();
        return t;
    }

    pub fn get_postion(&self) -> (usize, usize) {
        self.0.data.get_postion()
    }

    pub fn get_sign_table(&mut self)->& mut S{
        self.1
    }
    pub fn into_child(&mut self){
        self.1.enter()
    }
    pub fn leave_child(&mut self)->Option<()>{
        self.1.leave();
        Some(())
    }
}
