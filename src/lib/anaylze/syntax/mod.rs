use std::fmt::{Debug, Display};

use crate::lib::anaylze::lexical::expr::ExprLexical;

use super::{PreviewIter, Var};

mod expr;
mod literal;

pub trait SyntaxLoadNext<'a, I, T, L>
where
    I: PreviewIter<Item = L>,
{
    fn load_next(last: L, expr: &mut I) -> Result<LoadStatus<T, L>, LoadErr>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadStatus<T, N> {
    Success(T),
    NotMatch(N),
}

impl<T, N> LoadStatus<T, N> {
    pub fn ok(data: T) -> Self {
        LoadStatus::Success(data)
    }
    pub fn unmatch(expr: N) -> Self {
        LoadStatus::NotMatch(expr)
    }
    pub fn load_or<F>(self, f: F) -> T
    where
        F: Fn(N) -> T,
    {
        match self {
            LoadStatus::Success(t) => t,
            LoadStatus::NotMatch(e) => f(e),
        }
    }

    pub fn ok_or_else<F>(self, f: F) -> Result<T, LoadErr>
    where
        F: Fn(N) -> LoadErr,
    {
        match self {
            LoadStatus::Success(data) => Ok(data),
            LoadStatus::NotMatch(expr) => Err(f(expr)),
        }
    }
    pub fn unmatch_do<F>(self, f: F)
    where
        F: Fn(N),
    {
        if let Self::NotMatch(n) = self {
            f(n)
        }
    }
    pub fn and_then<R, F>(self, f: F) -> LoadStatus<R, N>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            LoadStatus::Success(data) => LoadStatus::ok(f(data)),
            LoadStatus::NotMatch(e) => LoadStatus::unmatch(e),
        }
    }

    pub fn into_ok<E>(self) -> Result<LoadStatus<T, N>, E> {
        Ok(self)
    }

    pub fn unwrap(self) -> T
    where
        N: Debug,
    {
        match self {
            LoadStatus::Success(data) => data,
            LoadStatus::NotMatch(e) => panic!("Failure to Unwrap=> {:?}", &e),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadErr {
    IterEnd,
    UnexprectLetical(String),
    UnSupportOperate(String),
}

impl LoadErr {
    pub fn unexpect<'a, T: Display>(expect: &'a str, get: T, pos: (usize, usize)) -> LoadErr {
        let (line, offset) = pos;
        LoadErr::UnexprectLetical(format!(
            "Expect `{}` But Get `{}` At line: {} Offset: {}",
            expect, get, line, offset
        ))
    }
    pub fn unsupport<'a>(var: &Var, operate: &'a str, pos: (usize, usize)) -> LoadErr {
        let (line, offset) = pos;
        LoadErr::UnSupportOperate(format!(
            "Value:[name: `{}` , value: {}] Can Not Be Op<{}> At line: {} Offset: {}",
            var.name, var.value, operate, line, offset
        ))
    }
}
