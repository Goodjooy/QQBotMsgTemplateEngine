mod sign;

use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{Sign},
            util::check_tag_name,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

use super::LiteralTag;

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for LiteralTag
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(_) = check_tag_name(&last, "endl", true) {
            Ok(LoadStatus::ok(Self::Endl))
        } else if let Some(_) = check_tag_name(&last, "sign", true) {
            Ok(Sign::load_next(last, expr)?.and_then(|sign| Self::Sign(sign)))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
