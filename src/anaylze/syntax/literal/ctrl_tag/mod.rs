mod if_statement;
mod loops;
mod var;

use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            If,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    }, SignTableHandle,
};

use super::{structs::Var, CtrlTag, Loops};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for CtrlTag
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        let res = Var::load_next(last, expr)?
            .and_then(|var| Self::Var(var))
            .unmatch_then(|f| If::load_next(f, expr)?.and_then(|f| Self::If(f)).into_ok())?
            .unmatch_then(|l| {
                Loops::load_next(l, expr)?
                    .and_then(|l| Self::Loops(l))
                    .into_ok()
            })?;

        Ok(res)
    }
}
