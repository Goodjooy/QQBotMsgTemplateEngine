use crate::anaylze::{
    lexical::{LexicalType, OutDataLoader},
    syntax::{
        literal::{structs::Image, util::check_tag_name},
        LoadErr, LoadStatus, SyntaxLoadNext,
    },
    SignTableHandle,
};

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LexicalType> for Image
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<Self, LexicalType>, LoadErr> {
        if let Some(tag) = check_tag_name(&last, "img", true) {
            if tag.chcek_attr_exist("url") {
                let url = tag.get("url").unwrap().get_raw_owner();
                Ok(LoadStatus::ok(Self::Url(url)))
            } else if tag.chcek_attr_exist("file") {
                let path = tag.get("file").unwrap().get_raw_owner();
                Ok(LoadStatus::ok(Self::File(path)))
            } else if tag.chcek_attr_exist("base64") {
                let base64 = tag.get("base64").unwrap().get_raw_owner();
                Ok(LoadStatus::ok(Self::Base64(base64)))
            } else {
                Err(LoadErr::attr_not_found(
                    "url OR path OR base64",
                    tag.get_name(),
                    expr.get_postion(),
                ))
            }
        } else {
            Ok(LoadStatus::unmatch(last))
        }
    }
}
