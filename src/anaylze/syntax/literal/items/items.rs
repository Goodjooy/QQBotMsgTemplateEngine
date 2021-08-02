use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{Item, ItemMeta, Items},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Items
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let meta = ItemMeta::load_next(last, expr)?.ok_or_else(|l| {
            LoadErr::unexpect("Tags In Syntax Or Literal", l, expr.get_postion())
        })?;
        let last = expr.next();
        let item = match last {
            Some(last) => Item::load_next(last, expr)?.ok_or_else(|l| {
                LoadErr::unexpect("Tags In Syntax Or Literal", l, expr.get_postion())
            })?,
            None => Item::Nil,
        };

        Ok(LoadStatus::ok(Self(meta, item)))
    }
}
