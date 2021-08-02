use crate::anaylze::{
    lexical::{tag::TagAttr, LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{Sign},
            util::check_tag_name,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Sign
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "sign", true) {
            let sign = {
                let s = tag
                    .get("sign")
                    .or(tag.get("s"))
                    .ok_or(LoadErr::attr_not_found(
                        "s OR sign",
                        tag.get_name(),
                        expr.get_postion(),
                    ))?
                    .get_raw_owner();
                s
            };
            let repeat: u32 = {
                let s = tag
                    .get("repeat")
                    .unwrap_or(TagAttr(String::from("1")))
                    .get_raw_owner();
                match s.parse() {
                    Ok(rep) => rep,
                    Err(err) => Err(LoadErr::unexpect("Usigned I32", err, expr.get_postion()))?,
                }
            };

            Ok(LoadStatus::ok(Self { sign, repeat }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
