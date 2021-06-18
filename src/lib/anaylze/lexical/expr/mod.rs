use super::{util::clear_space, PreviewableIter};
use crate::lib::anaylze::{LoadNextWithSignTable, Sign, SignTableHandle};

pub enum ExprLexical<'a> {
    Literal(String),
    CaculateSign(char),
    GroupSign(char),
    Digit(i64),
    Value(&'a Sign),
}

impl<'a> LoadNextWithSignTable<'a, ExprLexical<'a>> for ExprLexical<'a> {
    fn load_next<S>(data: &mut PreviewableIter, sign_table: &'a S) -> Option<ExprLexical<'a>>
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
            let value = sign_table.get_sign(&name)?;
            Some(Self::Value(value))
        }
    }
}

impl ExprLexical<'_> {
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
                let ch = data.preview()?;
                if ch.is_ascii_alphabetic() || ch == '_' || ch.is_digit(10) {
                    s.push(data.next()?);
                } else {
                    break Some(s);
                }
            }
        } else {
            None
        }
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
        let v=ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v,"_aaa");
        assert_eq!(data.preview().unwrap(),' ');

        clear_space(&mut data);
        let v=ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v,"abb_aa");
        assert_eq!(data.preview().unwrap(),' ');

        clear_space(&mut data);
        let v=ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v,"aaacd11");
        assert_eq!(data.preview().unwrap(),' ');

        clear_space(&mut data);
        let v=ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v,"aaAA");
        assert_eq!(data.preview().unwrap(),' ');

        clear_space(&mut data);
        let v=ExprLexical::read_sign_name(&mut data).unwrap();

        assert_eq!(v,"AAv");
        assert_eq!(data.preview().unwrap(),' ');

        clear_space(&mut data);
        let v=ExprLexical::read_sign_name(&mut data);

        assert_eq!(v,None);
        assert_eq!(data.next(),Some('0'));
    }

    #[test]
    fn full_test() {
        
    }
}
