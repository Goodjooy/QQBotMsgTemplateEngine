use super::{util::clear_space, PreviewableIter};
use crate::anaylze::{LoadNextWithSignTable, PreviewIter, Sign, SignTableHandle};
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprLexical {
    Nil,
    Literal(String),
    Bool(bool),
    CaculateSign(char),
    GroupSign(char),
    Digit(i64),
    Value(Sign),
}

impl<'a> LoadNextWithSignTable<'a, ExprLexical> for ExprLexical {
    fn load_next<S>(data: &mut PreviewableIter, sign_table: &'a mut S) -> Option<ExprLexical>
    where
        S: SignTableHandle,
    {
        clear_space(data);

        let ch = data.preview()?;
        if ch.is_digit(10) {
            data.next()?;
            let start = ch.to_digit(10).unwrap() as i64;
            let digit = Self::read_digit(start, data)?;
            Some(Self::Digit(digit))
        } else if ch.is_ascii_punctuation() {
            if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
                data.next()?;
                Some(Self::CaculateSign(ch))
            } else if ch == '(' || ch == ')' {
                data.next()?;
                Some(Self::GroupSign(ch))
            } else if ch == '\'' {
                data.next()?;
                let literal = Self::read_litral(data)?;
                Some(Self::Literal(literal))
            } else {
                None
            }
        } else {
            let name = Self::read_sign_name(data)?;
            if name=="true" || name=="false"{
                let v=if name=="true"{true}else{false};
                Some(Self::Bool(v))
            }else{

                let value = sign_table.get_sign(&name)?.clone();
                Some(Self::Value(value))
            }
        }
    }
}

impl ExprLexical {
    fn read_digit(init: i64, data: &mut PreviewableIter) -> Option<i64> {
        let mut num: i64 = init;
        loop {
            let ch = data.preview().or(Some('\0'))?;
            if ch.is_digit(10) {
                data.next()?;
                let t = ch.to_digit(10).unwrap() as i64;
                num = num * 10 + t;
            } else {
                break Some(num);
            }
        }
    }
    fn read_litral(data: &mut PreviewableIter) -> Option<String> {
        let mut s: String = String::new();
        loop {
            let ch = data.preview()?;
            if ch != '\'' {
                data.next()?;
                s.push(ch);
            } else {
                if data.next()? == '\'' {
                    break Some(s);
                } else {
                    break None;
                }
            }
        }
    }

    fn read_sign_name(data: &mut PreviewableIter) -> Option<String> {
        let mut s: String = String::new();
        let ch = data.preview().or(Some('\0'))?;
        if ch.is_ascii_alphabetic() || ch == '_' {
            s.push(data.next()?);
            loop {
                let need_end = data
                    .preview()
                    .and_then(|ch| {
                        if ch.is_ascii_alphabetic() || ch == '_' || ch.is_digit(10) {
                            s.push(data.next()?);
                            Some(false)
                        } else {
                            Some(true)
                        }
                    })
                    .or(Some(true))
                    .unwrap();

                if need_end {
                    break Some(s);
                }
            }
        } else {
            None
        }
    }
}
impl Display for ExprLexical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprLexical::Nil => write!(f, "UnKonw"),
            ExprLexical::Literal(l) => write!(f, "<literal, '{}'>", l),
            ExprLexical::CaculateSign(ch) => write!(f, "< {} >", ch),
            ExprLexical::GroupSign(ch) => write!(f, "< {} >", ch),
            ExprLexical::Digit(num) => write!(f, "< const, {} >", num),
            ExprLexical::Value(var) => match var {
                Sign::Var(v) => write!(f, "< '{}', {} >", v.name, v.value.to_string()),
                Sign::Const(_) => todo!(),
            },
            ExprLexical::Bool(b) => write!(f,"<bool, {}>",b),
        }
    }
}

pub struct ExprIter<'a, S>(PreviewableIter<'a>, &'a mut S, ExprLexical)
where
    S: SignTableHandle;

impl<'a, S> Iterator for ExprIter<'a, S>
where
    S: SignTableHandle,
{
    type Item = ExprLexical;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.2.clone();
        self.2 = ExprLexical::load_next(&mut self.0, self.1).or(Some(ExprLexical::Nil))?;

        if temp == ExprLexical::Nil {
            None
        } else {
            Some(temp)
        }
    }
}
impl<'a, S> PreviewIter for ExprIter<'a, S>
where
    S: SignTableHandle,
{
    fn preview(&self) -> Option<Self::Item> {
        let temp = self.2.clone();
        if ExprLexical::Nil == temp {
            None
        } else {
            Some(temp)
        }
    }
}

impl<'a, S: SignTableHandle> ExprIter<'a, S> {
    pub fn new(signs: &'a mut S, iter: PreviewableIter<'a>) -> Self {
        let mut t = ExprIter(iter, signs, ExprLexical::Nil);
        t.next();
        t
    }
    pub fn get_postion(&self) -> (usize, usize) {
        self.0.get_postion()
    }
}

#[cfg(test)]
mod expr {
    use super::*;

    #[test]
    fn test_read_digit() {
        let mut data = PreviewableIter::new("113+112");
        let init = data.next().unwrap().to_digit(10).unwrap() as i64;
        let n113 = ExprLexical::read_digit(init, &mut data).unwrap();

        assert_eq!(n113, 113);
        assert_eq!(data.next().unwrap(), '+');

        let init = data.next().unwrap().to_digit(10).unwrap() as i64;
        let n112 = ExprLexical::read_digit(init, &mut data).unwrap();

        assert_eq!(n112, 112);
        assert_eq!(data.next(), None);
    }

    #[test]
    fn test_read_literal() {
        let mut data = PreviewableIter::new("'abab''bbbb'");
        data.next().unwrap();
        let lit = ExprLexical::read_litral(&mut data).unwrap();

        assert_eq!(lit, "abab");
        assert_eq!(data.preview().unwrap(), '\'');
        data.next().unwrap();
        let lit = ExprLexical::read_litral(&mut data).unwrap();

        assert_eq!(lit, "bbbb");
        assert_eq!(data.next(), None);
    }

    #[test]
    fn test_read_sign() {
        let mut data = PreviewableIter::new("_aaa abb_aa aaacd11 aaAA AAv 011a");

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v, "_aaa");
        assert_eq!(data.preview().unwrap(), ' ');

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v, "abb_aa");
        assert_eq!(data.preview().unwrap(), ' ');

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v, "aaacd11");
        assert_eq!(data.preview().unwrap(), ' ');

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v, "aaAA");
        assert_eq!(data.preview().unwrap(), ' ');

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v, "AAv");
        assert_eq!(data.preview().unwrap(), ' ');

        clear_space(&mut data);
        let v = ExprLexical::read_sign_name(&mut data);

        assert_eq!(v, None);
        assert_eq!(data.next(), Some('0'));
    }

    #[test]
    fn full_test() {}
}
