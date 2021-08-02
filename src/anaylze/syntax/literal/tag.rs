use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{LoadErr, LoadStatus, SyntaxLoadNext},
    SignTableHandle,
};

use super::{CtrlTag, InfoTag, LiteralTag, Tag};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Tag
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = LiteralTag::load_next(last, expr)?
            .and_then(|li| Self::Liter(li))
            .unmatch_then(|info| {
                InfoTag::load_next(info, expr)?
                    .and_then(|info| Self::Info(info))
                    .into_ok()
            })?
            .unmatch_then(|ctrl| {
                CtrlTag::load_next(ctrl, expr)?
                    .and_then(|ctrl| Self::Ctrl(ctrl))
                    .into_ok()
            })?;

        Ok(res)
    }
}
