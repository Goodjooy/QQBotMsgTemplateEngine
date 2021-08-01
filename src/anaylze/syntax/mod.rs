use std::fmt::{Debug, Display};

use crate::anaylze::lexical::expr::ExprLexical;

use super::{PreviewIter, Var};

pub mod expr;
pub mod literal;

pub trait SyntaxLoadNext<'a, I, L>
where
    I: PreviewIter<Item = L>,
    Self: Sized,
{
    fn load_next(last: L, expr: &mut I) -> Result<LoadStatus<Self, L>, LoadErr>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadStatus<T, N> {
    Success(T),
    NotMatch(N),
}

impl<T, N> LoadStatus<T, N> {
    pub fn is_ok(&self) -> bool {
        match self {
            LoadStatus::Success(_) => true,
            LoadStatus::NotMatch(_) => false,
        }
    }

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
    pub fn unwarp_unmatch(self) -> N
    where
        T: Debug,
    {
        match self {
            LoadStatus::Success(d) => panic!("Failure to Unwrap=> {:?}", &d),
            LoadStatus::NotMatch(data) => data,
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LoadErr {
    IterEnd,
    UnexprectLetical(String),
    UnSupportOperate(String),
    TargetAttrNotExist(String),
    DataNotFoundInSignTable(String),
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
    pub fn attr_not_found<'a>(attr_name: &str, tag_name: &str, pos: (usize, usize)) -> LoadErr {
        let (line, offset) = pos;
        LoadErr::TargetAttrNotExist(format!(
            "Attr:[name: {}] Can Not Be Found In Tag[name: {}] At line: {} Offset: {}",
            attr_name, tag_name, line, offset
        ))
    }
    pub fn sign_not_in_table<'a>(sign_name: &str, pos: (usize, usize)) -> LoadErr {
        let (line, offset) = pos;
        LoadErr::DataNotFoundInSignTable(format!(
            "Sign:[name: {}] Can Not Be Found In Sign Table At line: {} Offset: {}",
            sign_name, line, offset
        ))
    }
}
