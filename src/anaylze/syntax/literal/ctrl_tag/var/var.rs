use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{ValueOperate, Var},
            util::check_tag_match,
            TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Var
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let name = {
                let na = tag.get("name").ok_or(LoadErr::attr_not_found(
                    "name",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                na.get_raw_owner()
            };
            let op = ValueOperate::new(tag, expr)?;

            Ok(LoadStatus::ok(Self { name, op }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}

impl TagInfo for Var {
    fn tag_name() -> &'static str {
        "var"
    }

    fn accept_full() -> bool {
        true
    }
}
