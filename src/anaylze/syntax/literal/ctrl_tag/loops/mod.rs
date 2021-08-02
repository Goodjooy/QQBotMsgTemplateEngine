mod for_loop;
mod r#loop;
mod while_loop;

use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{For, While}, Loop, Loops,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    }, SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Loops
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = Loop::load_next(last, expr)?
            .and_then(|l| Self::Loop(l))
            .unmatch_then(|l| {
                For::load_next(l, expr)?
                    .and_then(|f| Self::For(f))
                    .into_ok()
            })?
            .unmatch_then(|w| {
                While::load_next(w, expr)?
                    .and_then(|w| Self::While(w))
                    .into_ok()
            })?;
        Ok(res)
    }
}
