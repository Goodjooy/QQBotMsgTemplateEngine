use crate::lib::anaylze::lexical::expr::{ExprIter, ExprLexical};
use crate::lib::anaylze::syntax::expr::{ExprVar, Expression};
use crate::lib::anaylze::syntax::{LoadErr, LoadStatus};
use crate::lib::anaylze::{syntax::SyntaxLoadNext, SignTableHandle};

use super::Factor;

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Factor<'a>,ExprLexical<'a>> for Factor<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus< Factor<'a>,ExprLexical<'a>>, LoadErr> {
        match last {
            ExprLexical::Digit(num) => Ok(LoadStatus::ok(Factor::Digit(num))),
            ExprLexical::Value(var) => match var {
                crate::lib::anaylze::Sign::Var(v) => Ok(LoadStatus::ok(Factor::Var(ExprVar(v)))),
            },
            ExprLexical::GroupSign(ch) => {
                if ch == '(' {
                    let now = expr.next().ok_or(LoadErr::IterEnd)?;

                    let exp = Expression::load_next(now, expr)?
                        .ok_or_else(|e| LoadErr::unexpect("Expression", e, expr.get_postion()))?;

                    let close = expr.next().ok_or(LoadErr::IterEnd)?;
                    if let ExprLexical::GroupSign(ch) = close {
                        if ch == ')' {
                            Ok(LoadStatus::ok(Factor::SubExpr(Box::new(exp))))
                        } else {
                            Err(LoadErr::unexpect("')'", close, expr.get_postion()))
                        }
                    } else {
                        Err(LoadErr::unexpect("GroupSgin", close, expr.get_postion()))
                    }
                } else {
                    Err(LoadErr::unexpect("'('", ch, expr.get_postion()))
                }
            }
            ExprLexical::Nil => Err(LoadErr::IterEnd),
            default => Ok(LoadStatus::NotMatch(default)),
        }
    }
}

impl<'a> Factor<'a> {
    pub fn can_caculate<'b>(
        self,
        op: &'b str,
        pos: (usize, usize),
    ) -> Result<LoadStatus< Factor<'a>,ExprLexical<'a>>, LoadErr> {
        match self {
            Factor::SubExpr(_) | Factor::Digit(_) => Ok(LoadStatus::ok(self)),

            Factor::Var(v) => {
                let ExprVar(sign) = v;
                let value = &sign.value;
                match value {
                    crate::lib::anaylze::Value::UnSet | crate::lib::anaylze::Value::Int(_) => {
                        Ok(LoadStatus::ok(Factor::Var(ExprVar(sign))))
                    }
                    crate::lib::anaylze::Value::Str(_) | crate::lib::anaylze::Value::List(_) => {
                        Err(LoadErr::unsupport(sign, op, pos))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lib::anaylze::syntax::expr::{Caculate, Item, LexIter, SubCaculate, SubItem};
    use crate::lib::anaylze::{lexical::PreviewableIter, Value, Var};

    use super::*;

    #[test]
    fn test_load_digit() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();

        let t = Factor::load_next(last, &mut expr);
        assert_eq!(t, Ok(LoadStatus::Success(Factor::Digit(11))));
    }

    #[test]
    fn test_load_sign() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_D test_U test_S");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Factor::load_next(last, &mut expr);
        let d = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };
        assert_eq!(t, Ok(LoadStatus::Success(Factor::Var(ExprVar(&d)))));

        let last = expr.next().unwrap();
        let t = Factor::load_next(last, &mut expr);
        let d = Var {
            name: "".to_string(),
            value: Value::Int(11),
        };
        assert_eq!(t, Ok(LoadStatus::Success(Factor::Var(ExprVar(&d)))));

        let last = expr.next().unwrap();
        let t = Factor::load_next(last, &mut expr);
        let d = Var {
            name: "".to_string(),
            value: Value::Str("SSSS".to_string()),
        };
        assert_eq!(t, Ok(LoadStatus::Success(Factor::Var(ExprVar(&d)))));
    }
    #[test]
    fn test_sub() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("(test_D+test_U*22)*test_S-12");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Factor::load_next(last, &mut expr);

        let d = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };
        let i = Var {
            name: "".to_string(),
            value: Value::Int(11),
        };
        
        assert_eq!(
            t,
            Ok(LoadStatus::Success(Factor::SubExpr(Box::new(
                Expression::Caculate(Caculate(
                    Item(Factor::Var(ExprVar(&d)), SubItem::Nil),
                    SubCaculate::Addition(
                        Item(
                            Factor::Var(ExprVar(&i)),
                            SubItem::Multiple(Factor::Digit(22), Box::new(SubItem::Nil))
                        ),
                        Box::new(SubCaculate::Nil)
                    )
                ))
            ))))
        )
    }
}
