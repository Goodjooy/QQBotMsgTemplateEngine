use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{CmpMod, While},
            util::{check_end_tag, check_tag_name},
            Item, ItemMeta, Items, Literal,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for While
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

            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "while", expr.get_postion())?;

            Ok(LoadStatus::ok(While {
                model: cmp,
                body: Box::new(body),
            }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
