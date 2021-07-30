use crate::anaylze::syntax::expr::Literal;
use crate::anaylze::{lexical::expr::ExprIter, syntax::SyntaxLoadNext, SignTableHandle};
use crate::anaylze::lexical::expr::ExprLexical::{self};
use crate::anaylze::syntax::{LoadErr, LoadStatus};


impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>,ExprLexical> for Literal
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus< Literal,ExprLexical>, LoadErr> {
        if let ExprLexical::Literal(s) = last {
            Ok(LoadStatus::ok(Literal(s)))
        } else {
            Err(LoadErr::unexpect("Literal", last, expr.get_postion()))
        }
    }
}
