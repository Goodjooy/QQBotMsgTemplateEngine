use anaylze::syntax::LoadErr;

use crate::lib::anaylze::{self, SignTableHandle, lexical::expr::{ExprIter, ExprLexical}, syntax::{LoadStatus, SyntaxLoadNext}};

use super::Expression;

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, Expression<'a>> for Expression<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical<'a>,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<'a, Expression<'a>>, LoadErr> {
        match last {
            ExprLexical::Nil => Err(LoadErr::IterEnd),
            ExprLexical::Literal(_) => todo!(),
            ExprLexical::CaculateSign(_) => todo!(),
            ExprLexical::GroupSign(_) => todo!(),
            ExprLexical::Digit(_) => todo!(),
            ExprLexical::Value(_) => todo!(),
        }
    }
}
