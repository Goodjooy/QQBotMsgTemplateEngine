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
            ExprLexical::Literal(_) => Literal::load_next(last, expr)
                .and_then(|f| Ok(f.and_then(|t| Expression::Literal(t)))),
            ExprLexical::GroupSign(_) | ExprLexical::Digit(_) | ExprLexical::Value(_) => {
                Caculate::load_next(last, expr)
                    .and_then(|f| Ok(f.and_then(|f| Expression::Caculate(f))))
            }
            _ => Ok(LoadStatus::unmatch(last)),
        };

        ex
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lib::anaylze::syntax::expr::{ExprVar, Factor, Item, LexIter, SubCaculate, SubItem};
    use crate::lib::anaylze::Var;
    use crate::lib::anaylze::{PreviewableIter, Value};

    #[test]
    fn test_literal() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("'ababa'");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Expression::load_next(last, &mut expr);

        assert_eq!(
            t,
            Ok(LoadStatus::Success(Expression::Literal(Literal(
                "ababa".to_string()
            ))))
        )
    }
    #[test]
    fn test_value() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_D");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Expression::load_next(last, &mut expr);

        let v = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };
        assert_eq!(
            t,
            Ok(LoadStatus::Success(Expression::Caculate(Caculate(
                Item(Factor::Var(ExprVar(&v)), SubItem::Nil),
                SubCaculate::Nil
            ))))
        )
    }
    #[test]
    fn test_caculate() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_D+11-22*(5-2)");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Expression::load_next(last, &mut expr);

        let v = Var {
            name: "".to_string(),
            value: Value::Int(-11),
        };

        assert_eq!(
            t,
            Ok(LoadStatus::Success(Expression::Caculate(Caculate(
                Item(Factor::Var(ExprVar(&v)), SubItem::Nil),
                SubCaculate::Addition(
                    Item(Factor::Digit(11), SubItem::Nil),
                    Box::new(SubCaculate::Subtraction(
                        Item(
                            Factor::Digit(22),
                            SubItem::Multiple(
                                Factor::SubExpr(Box::new(Expression::Caculate(Caculate(
                                    Item(Factor::Digit(5), SubItem::Nil),
                                    SubCaculate::Subtraction(
                                        Item(Factor::Digit(2), SubItem::Nil),
                                        Box::new(SubCaculate::Nil)
                                    )
                                )))),
                                Box::new(SubItem::Nil)
                            ),
                        ),
                        Box::new(SubCaculate::Nil)
                    ))
                )
            ))))
        )
    }
}
