use std::fmt::Display;

use crate::lib::anaylze::lexical::expr::ExprLexical;

mod expr;
mod literal;

pub trait SyntaxLoadNext<'a, I, T>
where
    I: Iterator<Item = ExprLexical<'a>>,
{
    fn load_next(last: ExprLexical<'a>, expr: &mut I) -> Result<LoadStatus<'a, T>, LoadErr>;
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadStatus<'a, T> {
    Success(T),
    NotMatch(ExprLexical<'a>),
}

impl<'a, T> LoadStatus<'a, T> {
    pub fn ok(data: T) -> Self {
        LoadStatus::Success(data)
    }
    pub fn unmatch(expr: ExprLexical<'a>) -> Self {
        LoadStatus::NotMatch(expr)
    }
    pub fn load_or<F>(self, f: F) -> T
    where
        F: Fn(ExprLexical<'a>) -> T,
    {
        match self {
            LoadStatus::Success(t) => t,
            LoadStatus::NotMatch(e) => f(e),
        }
    }

    pub fn ok_or_else<F>(self, f: F) -> Result<T, LoadErr>
    where
        F: Fn(ExprLexical<'a>) -> LoadErr,
    {
        match self {
            LoadStatus::Success(data) => Ok(data),
            LoadStatus::NotMatch(expr) => Err(f(expr)),
        }
    }
    pub fn unmatch_do<F>(self, f: F)
    where
        F: Fn(ExprLexical<'a>),
    {
        if let Self::NotMatch(n) = self {
            f(n)
        }
    }
    pub fn and_then<R, F>(self, f: F) -> LoadStatus<'a, R>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            LoadStatus::Success(data) => LoadStatus::ok(f(data)),
            LoadStatus::NotMatch(e) => LoadStatus::unmatch(e),
        }
    }

    pub fn into_ok<E>(self)->Result<LoadStatus<'a,T>,E>{
        Ok(self)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadErr {
    IterEnd,
    UnexprectLetical(String),
}

impl LoadErr {
    pub fn unexpect<'a, T: Display>(expect: &'a str, get: T,pos:(usize,usize)) -> LoadErr {
        let(line,offset)=pos;
        LoadErr::UnexprectLetical(format!("Expect `{}` But Get `{}` At line: {} Offset: {}", expect, get,line,offset))
    }
}
