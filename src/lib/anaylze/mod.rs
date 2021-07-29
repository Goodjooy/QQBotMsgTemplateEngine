
use std::{fmt::Debug};

use self::lexical::PreviewableIter;
mod lexical;
mod sign_table;
mod syntax;
mod val_cmp;
mod val_format;

pub struct Anaylze<'a, S>
where
    S: SignTableHandle,
{
    sign_table: S,
    data: PreviewableIter<'a>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Sign {
    Var(Var),
}
#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub enum Value {
    UnSet,
    Int(i64),
    Str(String),
    List(Vec<Value>),
}
#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Var {
    pub name: String,
    pub value: Value,
}

pub trait SignTableHandle:Sized {
    fn check_exist(&self, key: &str) -> bool;
    fn get_sign(&self, key: &str) -> Option<&Sign>;
    fn get_mut_sign(&mut self, key: &str) -> Option<&mut Sign>;
    fn new_sign(&mut self, key: &str, value: Sign) -> Option<()>;

    fn leave(&mut self);
    fn enter(&mut self);
}

pub trait LoadNext<T> {
    fn load_next(data: &mut PreviewableIter) -> Option<T>;
}

pub trait LoadNextWithSignTable<'a, T> {
    fn load_next<S>(data: &mut PreviewableIter, sign_table: &'a mut S) -> Option<T>
    where
        S: SignTableHandle;
}

pub trait PreviewIter:Iterator {
    fn preview(&self)->Option<Self::Item>;
}