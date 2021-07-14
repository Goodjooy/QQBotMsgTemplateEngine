use crate::lib::anaylze::lexical::expr::{self, ExprIter, ExprLexical};
use crate::lib::anaylze::syntax::{LoadErr, LoadStatus, SyntaxLoadNext};
use crate::lib::anaylze::SignTableHandle;

use super::{nil_sign, Factor, Item, SubItem};

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Item<'a>> for Item<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, Item<'a>>, LoadErr> {
        //load factor
        let factor = Factor::load_next(last, expr)?
            .ok_or_else(|e| LoadErr::unexpect("Factor", e, expr.get_postion()))?;
        //load following
        expr.preview()
            //end iter can excepct
            .ok_or(LoadErr::IterEnd)
            .or(Ok(ExprLexical::Nil))
            //not expect sign can be accept
            .and_then(|last| SubItem::load_next(last, expr)?.into_ok())
            .or_else(|err| nil_sign(err, SubItem::Nil))?
            .and_then(|sub| Item(factor, sub))
            .into_ok()
    }
}

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, SubItem<'a>> for SubItem<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, SubItem<'a>>, LoadErr> {
        match last {
            ExprLexical::CaculateSign(sign) => match sign {
                '/' | '*' => {
                    //iter end and nil result
                    expr.next().ok_or(LoadErr::IterEnd)?;
                    expr.next()
                        .ok_or(LoadErr::IterEnd)
                        .and_then(|op| {
                            Factor::load_next(op, expr)?.ok_or_else(|exp| {
                                LoadErr::unexpect("Factor", exp, expr.get_postion())
                            })
                        })
                        .and_then(|f| {
                            expr.next()
                                .ok_or(LoadErr::IterEnd)
                                .or(Ok(ExprLexical::Nil))
                                .and_then(|e| Ok((f, e)))
                        })
                        .and_then(|last_expr| {
                            let (factor, last) = last_expr;
                            SubItem::load_next(last, expr).and_then(|d| Ok((d, factor)))
                        })
                        .and_then(|sub| {
                            let (sub, factor) = sub;
                            sub.and_then(|sub| SubItem::new(factor, sub, sign))
                                .into_ok()
                        })
                        .or_else(|err| nil_sign(err, SubItem::Nil))?
                        .into_ok()
                }
                _ => Ok(LoadStatus::ok(SubItem::Nil)),
            },
            ExprLexical::Nil => Ok(LoadStatus::ok(SubItem::Nil)),
            e => Ok(LoadStatus::unmatch(e)),
        }
    }
}

impl SubItem<'_> {
    fn new<'a>(factor: Factor<'a>, sub: SubItem<'a>, sign: char) -> SubItem<'a> {
        if sign == '/' {
            SubItem::Division(factor, Box::new(sub))
        } else {
            SubItem::Multiple(factor, Box::new(sub))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lib::anaylze::Value;
    use crate::{
        lib::anaylze::{
            lexical::PreviewableIter,
            syntax::{
                self,
                expr::{ExprVar, LexIter},
            },
            Var,
        },
        test_data,
    };

    #[test]
    fn test_signle_digit() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("13");
        let mut expr = ExprIter::new(&mut signs, iter);
        let last = expr.next().unwrap();
        let t = Item::load_next(last, &mut expr);

        assert_eq!(
            t,
            Ok(LoadStatus::Success(Item(Factor::Digit(13), SubItem::Nil)))
        );
    }

    #[test]
    fn test_operate_digit() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_D*11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Item::load_next(last, &mut expr);

        let v = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };
        assert_eq!(
            t,
            Ok(LoadStatus::Success(Item(
                Factor::Var(ExprVar(&v)),
                SubItem::Multiple(Factor::Digit(11), Box::new(SubItem::Nil))
            )))
        )
    }

    #[test]
    fn test_operate_digit_SS() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_D+11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Item::load_next(last, &mut expr);

        let v = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };
        assert_eq!(
            t,
            Ok(LoadStatus::Success(Item(
                Factor::Var(ExprVar(&v)),
                SubItem::Nil
            )))
        )
    }
}
