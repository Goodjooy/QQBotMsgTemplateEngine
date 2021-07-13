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
                        if ch == ')' {
                            Ok(LoadStatus::ok(Factor::SubExpr(Box::new(exp))))
                        } else {
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

#[cfg(test)]
mod test {
    use crate::lib::anaylze::syntax::expr::LexIter;
use crate::lib::anaylze::{Sign, Value, Var, lexical::PreviewableIter};

    use super::*;

    
    #[test]
    fn test_load_digit() {
        let mut signs=LexIter::new();
        let iter=PreviewableIter::new("11");
        let mut expr=ExprIter::new(&mut signs, iter);

        let last=expr.next().unwrap();

        let t=Factor::load_next(last, &mut expr);
        assert_eq!(t,Ok(LoadStatus::Success(Factor::Digit(11))));
    }

    #[test]
    fn test_load_sign() {
        let mut signs=LexIter::new();
        let iter=PreviewableIter::new("test_D test_U test_S");
        let mut expr=ExprIter::new(&mut signs, iter);

        
        let last=expr.next().unwrap();
        let t=Factor::load_next(last, &mut expr);
        let d=Var{name:"".to_string(),value:Value::Int(-11)};
        assert_eq!(t,Ok(LoadStatus::Success(Factor::Var(ExprVar(&d)))));

        let last=expr.next().unwrap();
        let t=Factor::load_next(last, &mut expr);
        let d=Var{name:"".to_string(),value:Value::Int(11)};
        assert_eq!(t,Ok(LoadStatus::Success(Factor::Var(ExprVar(&d)))));

        let last=expr.next().unwrap();
        let t=Factor::load_next(last, &mut expr);
        let d=Var{name:"".to_string(),value:Value::Str("SSSS".to_string())};
        assert_eq!(t,Ok(LoadStatus::Success(Factor::Var(ExprVar(&d))))); 
    }
    #[test]
    fn test_sub(){
        todo!()
    }
}
