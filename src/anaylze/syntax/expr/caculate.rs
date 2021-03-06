use crate::anaylze::{
    lexical::expr::{ExprIter, ExprLexical},
    syntax::{LoadErr, LoadStatus, SyntaxLoadNext},
    PreviewIter, SignTableHandle,
};

use super::{nil_sign, Caculate, Item, SubCaculate};

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, ExprLexical> for SubCaculate
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<SubCaculate, ExprLexical>, LoadErr> {
        println!("{}", last);
        if let ExprLexical::CaculateSign(sign) = last {
            match sign {
                '+' | '-' => {
                    expr.next().ok_or(LoadErr::IterEnd)?;
                    expr.next()
                        .ok_or(LoadErr::IterEnd)
                        .and_then(|last| {
                            Item::load_next(last, expr)?.ok_or_else(|exp| {
                                LoadErr::unexpect("Item", exp, expr.get_postion())
                            })
                        })
                        .and_then(|item| {
                            expr.preview()
                                .ok_or(LoadErr::IterEnd)
                                .or(Ok(ExprLexical::Nil))
                                .and_then(|f| Ok((item, f)))
                        })
                        .and_then(|last| {
                            let (item, last) = last;
                            SubCaculate::load_next(last, expr)?
                                .and_then(|sub| SubCaculate::new(sign, item, sub))
                                .into_ok()
                        })
                        .or_else(|err| nil_sign(err, SubCaculate::Nil))?
                        .into_ok()
                }
                _ => Err(LoadErr::unexpect("'+' Or '-'", last, expr.get_postion())),
            }
        } else if let ExprLexical::Nil = last {
            Ok(LoadStatus::Success(SubCaculate::Nil))
        } else {
            Ok(LoadStatus::Success(SubCaculate::Nil))
        }
    }
}

impl<'a, S> SyntaxLoadNext<'a, ExprIter<'a, S>, ExprLexical> for Caculate
where
    S: SignTableHandle,
{
    fn load_next(
        last: ExprLexical,
        expr: &mut ExprIter<'a, S>,
    ) -> Result<LoadStatus<Caculate, ExprLexical>, LoadErr> {
        let item = Item::load_next(last, expr)?
            .ok_or_else(|exp| LoadErr::unexpect("Item", exp, expr.get_postion()))?;
        expr.preview()
            .ok_or(LoadErr::IterEnd)
            .and_then(|last| SubCaculate::load_next(last, expr)?.into_ok())
            .or_else(|err| nil_sign(err, SubCaculate::Nil))?
            .and_then(|sub| Caculate(item, sub))
            .into_ok()
    }
}

impl<'a> SubCaculate {
    fn new(sign: char, item: Item, sub: SubCaculate) -> Self {
        if sign == '+' {
            SubCaculate::Addition(item, Box::new(sub))
        } else {
            SubCaculate::Subtraction(item, Box::new(sub))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::anaylze::{
        syntax::expr::{Factor, LexIter, SubItem},
        PreviewableIter,
    };

    #[test]
    fn test_digit() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let t = expr.next().unwrap();

        let v = Caculate::load_next(t, &mut expr);

        assert_eq!(
            v,
            Ok(LoadStatus::Success(Caculate(
                Item(Factor::Digit(11), SubItem::Nil),
                SubCaculate::Nil
            )))
        )
    }

    #[test]
    fn test_add_and_min() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("11+22-11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let t = expr.next().unwrap();
        println!("{:}", t);
        let v = Caculate::load_next(t, &mut expr);
        assert_eq!(
            v,
            Ok(LoadStatus::Success(Caculate(
                Item(Factor::Digit(11), SubItem::Nil),
                SubCaculate::Addition(
                    Item(Factor::Digit(22), SubItem::Nil),
                    Box::new(SubCaculate::Subtraction(
                        Item(Factor::Digit(11), SubItem::Nil),
                        Box::new(SubCaculate::Nil)
                    ))
                )
            )))
        )
    }

    #[test]
    fn test_add_and_min_and_higher() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("11+22*11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let t = expr.next().unwrap();
        println!("{:}", t);
        let v = Caculate::load_next(t, &mut expr);
        assert_eq!(
            v,
            Ok(LoadStatus::Success(Caculate(
                Item(Factor::Digit(11), SubItem::Nil),
                SubCaculate::Addition(
                    Item(
                        Factor::Digit(22),
                        SubItem::Multiple(Factor::Digit(11), Box::new(SubItem::Nil))
                    ),
                    Box::new(SubCaculate::Nil)
                )
            )))
        )
    }

    #[test]
    fn test_operate_unsupport() {
        let mut signs = LexIter::new();
        let iter = PreviewableIter::new("test_S+11");
        let mut expr = ExprIter::new(&mut signs, iter);

        let last = expr.next().unwrap();
        let t = Caculate::load_next(last, &mut expr);

        assert_eq!(
            t,
            Err(LoadErr::UnSupportOperate(
                "Value:[name: `` , value: SSSS] Can Not Be Op<+, -, *, /> At line: 0 Offset: 7"
                    .to_string()
            ))
        )
    }
}
