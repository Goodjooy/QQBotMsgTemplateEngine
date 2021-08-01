use crate::anaylze::{
    lexical::{tag::TagAttr, LexicalType, OutDataLoader},
    syntax::{
        literal::{structs::At, util::check_tag_name},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for At
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "at", true) {
            let uid: u64 = {
                let id = tag.get("uid").ok_or(LoadErr::attr_not_found(
                    "uid",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                let id_s = id.get_raw_owner();
                match id_s.parse() {
                    Ok(v) => v,
                    Err(e) => Err(LoadErr::unexpect("Unsigned Int64", e, expr.get_postion()))?,
                }
            };

            let sep = {
                tag.get("sep")
                    .unwrap_or(TagAttr(String::from("")))
                    .get_raw_owner()
            };
            Ok(LoadStatus::ok(Self { uid, sep }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
