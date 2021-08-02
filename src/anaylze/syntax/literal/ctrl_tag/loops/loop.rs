use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{
            util::{check_end_tag, check_tag_match, load_express},
            Item, ItemMeta, Items, Literal, Loop, TagInfo,
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
                        value: Value::Int(1),
                    }),
                );
            }
            //TODO: ItemsLoader
            let body: Items = Items(ItemMeta::Literal(Literal("test".to_string())), Item::Nil);

            //load end tag
            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, "loop", expr.get_postion())?;

            let res = Loop {
                times: times_expr,
                name: loop_time_name,
                body: Box::new(body),
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
