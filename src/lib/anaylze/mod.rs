use std::fmt::{Debug, Display};
mod val_format;
mod lexical;

pub enum Sign {
    Var(Var),
}
#[derive(PartialEq, PartialOrd)]
pub enum Value{
    Int(i32),
    Uint(u32),
    F32(f32),
    F64(f64),
    Str(String),
}

pub struct Var {
    name: String,
    var_type: &'static str,
    value: Value,
}

pub trait SignTableHandle<T>
{
    fn check_exist(&self, key: &str) -> bool;
    fn get_sign(&self, key: &str) -> Option<&Sign>;
    fn get_mut_sign(&mut self, key: &str) -> Option<&mut Sign>;
    fn new_sign(&mut self, key: &str, value: Sign) -> Option<()>;
}
