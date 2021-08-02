use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::For,
            util::{check_end_tag, check_tag_match, load_attr},
            Item, ItemMeta, Items, Literal, TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle, Value,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for For
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let pos = expr.get_postion();

            let source = {
                let table = expr.get_sign_table();
                let source = load_attr(tag, "source", pos)?;
                let source = table
                    .get_sign(source.get_raw())
                    .ok_or(LoadErr::sign_not_in_table(source.get_raw(), pos))?;
                if source.is_value() {
                    if let Some(Value::List(_)) = source.into_value() {
                    } else {
                        return Err(LoadErr::unexpect("List", source, pos));
                    }
                } else {
                    return Err(LoadErr::unexpect("Var", source, pos));
                }
                source.into_value().unwrap().clone()
            };

            let name = load_attr(tag, "name", pos)?.get_raw_owner();

            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);

            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "for", expr.get_postion())?;

            Ok(LoadStatus::ok(Self {
                source,
                name,
                body: Box::new(body),
            }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
impl TagInfo for For {
    fn tag_name() -> &'static str {
        "for"
    }
}
