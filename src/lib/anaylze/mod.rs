use std::fmt::{Debug, Display};

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
pub enum Sign {
    Var(Var),
}
#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Str(String),
    List(Vec<Value>),
}
#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Var {
    pub name: String,
    pub value: Value,
}

pub trait SignTableHandle {
    fn check_exist(&self, key: &str) -> bool;
    fn get_sign(&self, key: &str) -> Option<&Sign>;
    fn get_mut_sign(&mut self, key: &str) -> Option<&mut Sign>;
    fn new_sign(&mut self, key: &str, value: Sign) -> Option<()>;
}

pub trait LoadNext<T> {
    fn load_next(data: &mut PreviewableIter) -> Option<T>;
}

pub trait LoadNextWithSignTable<'a, T> {
    fn load_next<S>(data: &mut PreviewableIter, sign_table: &'a S) -> Option<T>
    where
        S: SignTableHandle;
}
