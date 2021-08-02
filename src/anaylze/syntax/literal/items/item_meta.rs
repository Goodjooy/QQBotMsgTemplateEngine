use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{ItemMeta, Tag},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for ItemMeta
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = String::load_next(last, expr)?
            .and_then(|lit| Self::Lit(lit))
            .unmatch_then(|tag| {
                Tag::load_next(tag, expr)?
                    .and_then(|tag| Self::Tag(tag))
                    .into_ok()
            })?;

        Ok(res)
    }
}
