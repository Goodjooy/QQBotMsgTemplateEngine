use crate::lib::anaylze::syntax::expr::{Caculate, Literal};
use anaylze::syntax::LoadErr;

use crate::lib::anaylze::{
    self,
    lexical::expr::{ExprIter, ExprLexical},
    syntax::{LoadStatus, SyntaxLoadNext},
    SignTableHandle,
};

use super::Expression;

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Expression<'a>> for Expression<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, Expression<'a>>, LoadErr> {
        let ex = match last {
            ExprLexical::Literal(lit) => Ok(LoadStatus::ok(Expression::Literal(Literal(lit)))),
            ExprLexical::GroupSign(_) | ExprLexical::Digit(_) => Caculate::load_next(last, expr)
                .and_then(|f| Ok(f.and_then(|f| Expression::Caculate(f)))),

            _ => Ok(LoadStatus::unmatch(last)),
        };

        ex
    }
}
