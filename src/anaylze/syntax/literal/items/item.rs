use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{Item, ItemMeta},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Item
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = ItemMeta::load_next(last, expr)?.ok_or_else(|err| {
            LoadErr::unexpect("Tags In Syntax Or Literal", err, expr.get_postion())
        })?;

        let item = {
            match expr.next() {
                Some(i) => Item::load_next(i, expr)?,
                None => LoadStatus::ok(Self::Nil),
            }
        }
        .ok_or_else(|err| {
            LoadErr::unexpect("Tags In Syntax Or Literal", err, expr.get_postion())
        })?;

        Ok(LoadStatus::ok(Self::Item(res, Box::new(item))))
    }
}
