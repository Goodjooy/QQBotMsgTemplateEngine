use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{CmpMod, IfFollows},
            util::{check_tag_match, load_body},
            If, TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    PreviewIter, SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for If
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let model = {
                let ty_attr = tag.get("mod").ok_or(LoadErr::attr_not_found(
                    "mod",
                    tag.get_name(),
                    expr.get_postion(),
                ))?;
                let ty = ty_attr.get_raw();

                CmpMod::new(ty, tag, expr.get_postion(), expr.get_sign_table())?
            };

            expr.into_child();

            let last = expr.next().ok_or(LoadErr::IterEnd)?;
            let body = load_body(last, expr, Self::tag_name())?;

            expr.leave_child();

            let follows = {
                let da = expr.preview();
                match da {
                    Some(s) => IfFollows::load_next(s, expr)?,
                    None => LoadStatus::ok(IfFollows::Nil),
                }
            }
            .ok_or_else(|f| LoadErr::unexpect("If Flow Tag", f, expr.get_postion()))?;

            Ok(LoadStatus::ok(Self {
                model,
                body,
                follows,
            }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
impl TagInfo for If {
    fn tag_name() -> &'static str {
        "if"
    }
}
