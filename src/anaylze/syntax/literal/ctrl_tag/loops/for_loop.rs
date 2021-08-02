use crate::anaylze::{Sign, SignTableHandle, Value, Var, lexical::{LexicalType, OutDataLoader}, syntax::{
        literal::{
            structs::For,
            util::{check_tag_match, load_attr, load_body}, TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    }};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for For
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let pos = expr.get_postion();

            let source = {
                let table = expr.get_sign_table();
                let source = load_attr(tag, "source", pos)?;
                let source = table
                    .get_sign(source.get_raw())
                    .ok_or(LoadErr::sign_not_in_table(source.get_raw(), pos))?;
                if source.is_value() {
                    if let Some(Value::List(_)) = source.into_value() {
                    } else {
                        return Err(LoadErr::unexpect("List", source, pos));
                    }
                } else {
                    return Err(LoadErr::unexpect("Var", source, pos));
                }
                source.into_value().unwrap().clone()
            };

            let name = load_attr(tag, "name", pos)?.get_raw_owner();

            expr.into_child();
            {
                let table = expr.get_sign_table();
                table.new_sign(
                    &name,
                    Sign::Var(Var {
                        name: name.clone(),
                        value: Value::UnSet(name.clone()),
                    }),
                );
            }

            let last = expr.next().ok_or(LoadErr::IterEnd)?;
            let body = load_body(last, expr, Self::tag_name())?;

            expr.leave_child();

            Ok(LoadStatus::ok(Self { source, name, body }))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
impl TagInfo for For {
    fn tag_name() -> &'static str {
        "for"
    }
}
