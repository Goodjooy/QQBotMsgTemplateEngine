mod at;
mod img;

use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{literal::structs::Image, LoadErr, LoadStatus, SyntaxLoadNext},
    SignTableHandle,
};

use super::{structs::At, util::check_tag_name, InfoTag};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for InfoTag
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(_) = check_tag_name(&last, "img", true) {
            Ok(Image::load_next(last, expr)?.and_then(|img| Self::Img(img)))
        } else if let Some(_) = check_tag_name(&last, "at", true) {
            Ok(At::load_next(last, expr)?.and_then(|at| Self::At(at)))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
