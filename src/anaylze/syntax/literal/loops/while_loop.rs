use crate::anaylze::lexical::LexicalType;
use crate::anaylze::lexical::OutDataLoader;
use crate::anaylze::syntax::literal::structs::CmpMod;
use crate::anaylze::syntax::literal::structs::While;
use crate::anaylze::syntax::literal::util::check_tag_name;
use crate::anaylze::syntax::literal::Item;
use crate::anaylze::syntax::literal::ItemMeta;
use crate::anaylze::syntax::literal::Items;
use crate::anaylze::syntax::literal::Literal;
use crate::anaylze::syntax::LoadErr;
use crate::anaylze::syntax::LoadStatus;
use crate::anaylze::syntax::SyntaxLoadNext;
use crate::anaylze::SignTableHandle;

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for While<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<crate::anaylze::syntax::LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "while", false) {
            let cmp = {
                let cmp_mod = tag.get("mod").ok_or(LoadErr::attr_not_found(
                    "mod",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                let ty = cmp_mod.get_raw();
                CmpMod::new(ty, tag, expr.get_postion(), expr.get_sign_table())?
            };

            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);

            Ok(LoadStatus::ok(While {
                model: cmp,
                body: Box::new(body),
            }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
