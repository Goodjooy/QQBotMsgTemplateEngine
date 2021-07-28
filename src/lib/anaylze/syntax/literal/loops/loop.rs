use crate::lib::anaylze::syntax::literal::Literal;
use crate::lib::anaylze::lexical::expr::ExprIter;
use crate::lib::anaylze::Sign;
use crate::lib::anaylze::Value;
use crate::lib::anaylze::Var;

use crate::lib::anaylze::lexical::LexicalType;
use crate::lib::anaylze::lexical::OutDataLoader;
use crate::lib::anaylze::syntax::expr::Expression;
use crate::lib::anaylze::syntax::literal::ItemMeta;
use crate::lib::anaylze::syntax::literal::Items;
use crate::lib::anaylze::syntax::literal::ulit::check_tag_name;
use crate::lib::anaylze::syntax::literal::Item;
use crate::lib::anaylze::syntax::literal::Loop;
use crate::lib::anaylze::syntax::LoadErr;
use crate::lib::anaylze::syntax::LoadStatus;
use crate::lib::anaylze::syntax::SyntaxLoadNext;
use crate::lib::anaylze::SignTableHandle;

impl<'a, S: SignTableHandle> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Loop<'a> {
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "loop", false) {
            let times_literal = tag.get("times").ok_or(LoadErr::attr_not_found(
                "times",
                tag.get_name(),
                expr.get_postion(),
            ))?;
            let expr_iter = times_literal.get_iter();
            let mut expr_iter = ExprIter::new(expr.get_sign_table(), expr_iter);
            let expr_last = expr_iter.next().ok_or(LoadErr::IterEnd)?;
            let times_expr = Expression::load_next(expr_last, &mut expr_iter)?
                .ok_or_else(|f| LoadErr::unexpect("Expression", f, expr_iter.get_postion()))?;

            let loop_time_name = tag.get("name").and_then(|f| Some(f.0));
            //load loop body
            expr.into_child();
            if let Some(name) = &loop_time_name {
                expr.get_sign_table().borrow_mut().new_sign(
                    name,
                    Sign::Var(Var {
                        name: name.to_string(),
                        value: Value::Int(1),
                    }),
                );
            }
            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())),Item::Nil);
            let res = Loop {
                times: times_expr,
                name: loop_time_name,
                body: Box::new(body),
            };
            Ok(LoadStatus::ok(res))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
