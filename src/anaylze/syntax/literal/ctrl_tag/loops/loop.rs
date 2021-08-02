use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            util::{check_tag_match, load_body, load_express}, Loop, TagInfo,
        },
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    Sign, SignTableHandle, Value, Var,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Loop
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_match::<Self>(&last) {
            let times_expr = load_express(tag, "times", expr.get_postion(), expr.get_sign_table())?;

            let loop_time_name = tag.get("name").and_then(|f| Some(f.get_raw_owner()));
            //load loop body
            expr.into_child();
            if let Some(name) = &loop_time_name {
                expr.get_sign_table().new_sign(
                    name,
                    Sign::Var(Var {
                        name: name.to_string(),
                        value: Value::UnSet(name.to_string()),
                    }),
                );
            } //TODO: 每次碰到循环语句就进入符号表子表，会出现未预期的行为

            let last = expr.next().ok_or(LoadErr::IterEnd)?;
            let body = load_body(last, expr, Self::tag_name())?;

            // leave sign table
            expr.leave_child();

            

            let res = Loop {
                times: times_expr,
                name: loop_time_name,
                body,
            };
            Ok(LoadStatus::ok(res))
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}

impl TagInfo for Loop {
    fn tag_name() -> &'static str {
        "loop"
    }
}
