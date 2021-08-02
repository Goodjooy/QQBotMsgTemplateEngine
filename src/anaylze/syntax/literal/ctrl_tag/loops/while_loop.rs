use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{CmpMod, While},
            util::{check_tag_match, load_body},
            TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for While
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<crate::anaylze::syntax::LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let cmp = {
                let cmp_mod = tag.get("mod").ok_or(LoadErr::attr_not_found(
                    "mod",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                let ty = cmp_mod.get_raw();
                CmpMod::new(ty, tag, expr.get_postion(), expr.get_sign_table())?
            };

            expr.into_child();

            let last = expr.next().ok_or(LoadErr::IterEnd)?;
            let body = load_body(last, expr, Self::tag_name())?;

            expr.leave_child();

            Ok(LoadStatus::ok(While { model: cmp, body }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
impl TagInfo for While {
    fn tag_name() -> &'static str {
        "while"
    }
}
