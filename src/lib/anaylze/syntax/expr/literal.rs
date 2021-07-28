use crate::lib::anaylze::syntax::expr::Literal;
use crate::lib::anaylze::{lexical::expr::ExprIter, syntax::SyntaxLoadNext, SignTableHandle};
use lib::anaylze::lexical::expr::ExprLexical::{self};
use lib::anaylze::syntax::{LoadErr, LoadStatus};

use crate::lib::{self};

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
