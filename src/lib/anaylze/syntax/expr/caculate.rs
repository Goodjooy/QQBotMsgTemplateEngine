use crate::lib::anaylze::lexical::expr::{ExprIter, ExprLexical};
use crate::lib::anaylze::syntax::{LoadErr, LoadStatus};
use crate::lib::anaylze::{syntax::SyntaxLoadNext, SignTableHandle};

use super::{nil_sign, Caculate, Item, SubCaculate};

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, SubCaculate<'a>> for SubCaculate<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, SubCaculate<'a>>, LoadErr> {
        if let ExprLexical::CaculateSign(sign) = last {
            if sign == '+' || sign == '-' {
                expr.next()
                    .ok_or(LoadErr::IterEnd)
                    .and_then(|last| {
                        let item = Item::load_next(last, expr)?
                            .ok_or_else(|exp| LoadErr::unexpect("Item", exp))?;
                        expr.next()
                            .ok_or(LoadErr::IterEnd)
                            .and_then(|last| {
                                SubCaculate::load_next(last, expr)?
                                    .and_then(|sub| SubCaculate::new(sign, item.clone(), sub))
                                    .into_ok()
                            })
                            .or_else(|err| nil_sign(err, SubCaculate::Nil))?
                            .into_ok()
                    })
                    .or_else(|err| nil_sign(err, SubCaculate::Nil))?
                    .into_ok()
            } else {
                Err(LoadErr::unexpect("'+' Or '-'", last))
            }
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Caculate<'a>> for Caculate<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, Caculate<'a>>, LoadErr> {
        let item = Item::load_next(last, expr)?.ok_or_else(|exp| LoadErr::unexpect("Item", exp))?;

        expr.next()
            .ok_or(LoadErr::IterEnd)
            .and_then(|last| SubCaculate::load_next(last, expr)?.into_ok())
            .or_else(|err| nil_sign(err, SubCaculate::Nil))?
            .and_then(|sub| Caculate(item.clone(), sub))
            .into_ok()
    }
}

impl<'a> SubCaculate<'a> {
    fn new(sign: char, item: Item<'a>, sub: SubCaculate<'a>) -> Self {
        if sign == '+' {
            SubCaculate::Addition(item, Box::new(sub))
        } else {
            SubCaculate::Subtraction(item, Box::new(sub))
        }
    }
}
