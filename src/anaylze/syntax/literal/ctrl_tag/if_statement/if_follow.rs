use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            structs::{CmpMod, If, IfFollows},
            util::{check_tag_name, load_body},
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    PreviewIter, SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for IfFollows
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "elif", false) {
            let _last = expr.next().ok_or(LoadErr::IterEnd)?;
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
            let body = load_body(last, expr, "elif")?;

            expr.leave_child();

            let follows = {
                let da = expr.preview();
                match da {
                    Some(s) => IfFollows::load_next(s, expr)?,
                    None => LoadStatus::ok(IfFollows::Nil),
                }
            }
            .ok_or_else(|f| LoadErr::unexpect("If Flow Tag", f, expr.get_postion()))?;

            Ok(LoadStatus::ok(Self::Elif(Box::new(If {
                model,
                body,
                follows,
            }))))
        } else if let Some(_) = check_tag_name(&last, "else", false) {
            let _last = expr.next().ok_or(LoadErr::IterEnd)?;

            expr.into_child();

            let last = expr.next().ok_or(LoadErr::IterEnd)?;
            let body = load_body(last, expr, "else")?;

            expr.leave_child();

            Ok(LoadStatus::ok(Self::Else(body)))
        } else {
            Ok(LoadStatus::ok(Self::Nil))
        }
    }
}
