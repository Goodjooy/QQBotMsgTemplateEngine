use crate::lib::anaylze::syntax::literal::Literal;
use crate::lib::anaylze::Sign;
use crate::lib::anaylze::Value;
use crate::lib::anaylze::Var;

use crate::lib::anaylze::lexical::LexicalType;
use crate::lib::anaylze::lexical::OutDataLoader;
use crate::lib::anaylze::syntax::literal::util::check_end_tag;
use crate::lib::anaylze::syntax::literal::util::check_tag_name;
use crate::lib::anaylze::syntax::literal::util::load_express;
use crate::lib::anaylze::syntax::literal::Item;
use crate::lib::anaylze::syntax::literal::ItemMeta;
use crate::lib::anaylze::syntax::literal::Items;
use crate::lib::anaylze::syntax::literal::Loop;
use crate::lib::anaylze::syntax::LoadErr;
use crate::lib::anaylze::syntax::LoadStatus;
use crate::lib::anaylze::syntax::SyntaxLoadNext;
use crate::lib::anaylze::SignTableHandle;

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Loop<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "loop", false) {
            let times_expr = load_express(tag, "times", expr.get_postion(), expr.get_sign_table())?;

            let loop_time_name = tag.get("name").and_then(|f| Some(f.get_raw_owner()));
            //load loop body
            expr.into_child();
            if let Some(name) = &loop_time_name {
                expr.get_sign_table().new_sign(
                    name,
                    Sign::Var(Var {
                        name: name.to_string(),
                        value: Value::Int(1),
                    }),
                );
            }
            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);

            //load end tag
            let end_tag=expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "loop", expr.get_postion())?;

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
