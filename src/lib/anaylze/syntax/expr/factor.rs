use crate::lib::anaylze::lexical::expr::{ExprIter, ExprLexical};
use crate::lib::anaylze::syntax::expr::{ExprVar, Expression};
use crate::lib::anaylze::syntax::{LoadErr, LoadStatus};
use crate::lib::anaylze::{syntax::SyntaxLoadNext, SignTableHandle};

use super::Factor;

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Factor<'a>> for Factor<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, Factor<'a>>, LoadErr> {
        match last {
            ExprLexical::Digit(num) => Ok(LoadStatus::ok(Factor::Digit(num))),
            ExprLexical::Value(var) => match var {
                crate::lib::anaylze::Sign::Var(v) => Ok(LoadStatus::ok(Factor::Var(ExprVar(v)))),
            },
            ExprLexical::GroupSign(ch) => {
                if ch == '(' {
                    let now = expr.next().ok_or(LoadErr::IterEnd)?;

                    let exp = Expression::load_next(now, expr)?
                        .ok_or_else(|e| LoadErr::unexpect("Expression", e))?;

                    let close = expr.next().ok_or(LoadErr::IterEnd)?;
                    if let ExprLexical::GroupSign(ch) = close {
                        if ch==')'{
                            Ok(LoadStatus::ok(Factor::SubExpr(Box::new(exp))))
                        }else {
                            Err(LoadErr::unexpect("')'", close))
                        }
                    } else {
                        Err(LoadErr::unexpect("GroupSgin", close))
                    }
                } else {
                    Err(LoadErr::unexpect("'('", ch))
                }
            }
            ExprLexical::Nil => Err(LoadErr::IterEnd),
            default => Ok(LoadStatus::NotMatch(default)),
        }
    }
}
