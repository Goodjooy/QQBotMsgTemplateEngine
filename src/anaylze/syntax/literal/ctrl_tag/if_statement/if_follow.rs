use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{CmpMod, If, IfFollows},
            util::{check_end_tag, check_tag_name},
            Item, ItemMeta, Items, Literal,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    PreviewIter, SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for IfFollows
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "elif", false) {
            let _last = expr.next().ok_or(LoadErr::IterEnd)?;
            let model = {
                let ty_attr = tag.get("mod").ok_or(LoadErr::attr_not_found(
                    "mod",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                let ty = ty_attr.get_raw();

                CmpMod::new(ty, tag, expr.get_postion(), expr.get_sign_table())?
            };

            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);

            //close tag
            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "elif", expr.get_postion())?;

            let follows = {
                let da = expr.preview();
                match da {
                    Some(s) => IfFollows::load_next(s, expr)?,
                    None => LoadStatus::ok(IfFollows::Nil),
                }
            }
            .ok_or_else(|f| LoadErr::unexpect("If Flow Tag", f, expr.get_postion()))?;

            Ok(LoadStatus::ok(Self::Elif(Box::new(If {
                model: model,
                body: Box::new(body),
                follows: follows,
            }))))
        } else if let Some(_) = check_tag_name(&last, "else", false) {
            let _last = expr.next().ok_or(LoadErr::IterEnd)?;

            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);
            //close tag
            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "else", expr.get_postion())?;

            Ok(LoadStatus::ok(Self::Else(Box::new(body))))
        } else {
            Ok(LoadStatus::ok(Self::Nil))
        }
    }
}