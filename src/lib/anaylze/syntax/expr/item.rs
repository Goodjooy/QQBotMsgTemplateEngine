use crate::lib::anaylze::lexical::expr::{ExprIter, ExprLexical};
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
        let factor =
            Factor::load_next(last, expr)?.ok_or_else(|e| LoadErr::unexpect("Factor", e))?;
        expr.next()
            .ok_or(LoadErr::IterEnd)
            .and_then(|last| SubItem::load_next(last, expr)?.into_ok())
            .or_else(|err| nil_sign(err, SubItem::Nil))?
            .and_then(|sub| Item(factor.clone(), sub))
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
            ExprLexical::CaculateSign(sign) => {
                if sign == '/' || sign == '*' {
                    let factor = Factor::load_next(last, expr)?
                        .ok_or_else(|e| LoadErr::unexpect("Factor", e))?;

                    //iter end and nil result
                    expr.next()
                        .ok_or(LoadErr::IterEnd)
                        .and_then(|last_expr| {
                            SubItem::load_next(last_expr, expr)?
                                .and_then(|sub| {
                                    if sign == '/' {
                                        SubItem::Division(factor.clone(), Box::new(sub))
                                    } else {
                                        SubItem::Multiple(factor.clone(), Box::new(sub))
                                    }
                                })
                                .into_ok()
                        })
                        .or_else(|err| nil_sign(err, SubItem::Nil))?
                        .into_ok()
                } else {
                    Ok(LoadStatus::ok(SubItem::Nil))
                }
            }
            e => Ok(LoadStatus::unmatch(e)),
        }
    }
}
