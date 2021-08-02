use crate::anaylze::lexical::LexicalType;
use crate::anaylze::lexical::OutDataLoader;
use crate::anaylze::syntax::literal::structs::ValueOperate;
use crate::anaylze::syntax::literal::structs::Var;
use crate::anaylze::syntax::literal::util::check_tag_name;
use crate::anaylze::syntax::LoadErr;
use crate::anaylze::syntax::LoadStatus;
use crate::anaylze::syntax::SyntaxLoadNext;
use crate::anaylze::SignTableHandle;

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Var
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "var", true) {
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
