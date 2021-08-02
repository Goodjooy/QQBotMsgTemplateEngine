use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{LoadErr, LoadStatus, SyntaxLoadNext},
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for String
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        _expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let LexicalType::Literal(l) = last {
            let s = l.0;
            Ok(LoadStatus::ok(s))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
