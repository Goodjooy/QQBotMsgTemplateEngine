mod at;
mod img;

use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{structs::Image},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

use super::{structs::At, InfoTag};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for InfoTag
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = Image::load_next(last, expr)?;
        if res.is_ok() {
            return Ok(LoadStatus::ok(Self::Img(res.unwrap())));
        } else {
            let res = At::load_next(res.unwarp_unmatch(), expr)?;
            if res.is_ok() {
                return Ok(LoadStatus::ok(Self::At(res.unwrap())));
            } else {
                return Ok(LoadStatus::unmatch(res.unwarp_unmatch()));
            }
        }
    }
}
