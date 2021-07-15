use std::error::Error;

use super::LiteralTag;
use crate::lib::anaylze::lexical::tag::Tag::CloseTag;
use crate::lib::anaylze::lexical::tag::Tag::FullTag;
use crate::lib::anaylze::lexical::tag::Tag::StartTag;
use crate::lib::anaylze::lexical::tag::TagStruct;
use crate::lib::anaylze::lexical::LexicalType;
use crate::lib::anaylze::lexical::OutDataLoader;
use crate::lib::anaylze::syntax::literal::structs::Sign;
use crate::lib::anaylze::syntax::LoadErr;
use crate::lib::anaylze::syntax::{LoadStatus, SyntaxLoadNext};
use crate::lib::anaylze::SignTableHandle;

impl<'a, S> SyntaxLoadNext<'a, OutDataLoader<'a, S>, LiteralTag<'a>, LexicalType> for LiteralTag<'a>
where
    S: SignTableHandle,
{
    fn load_next(
        last: LexicalType,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<LoadStatus<LiteralTag<'a>, LexicalType>, LoadErr> {
        match last {
            LexicalType::Tag(tag) => match tag {
                //关闭标签
                FullTag(ft) => (load_by_tag_type(ft, true, expr, expr.get_postion())),
                StartTag(ft) => (load_by_tag_type(ft, false, expr, expr.get_postion())),
                CloseTag(_) => todo!(),
            },
            _ => Ok(LoadStatus::unmatch(last)),
        }
    }
}

fn load_by_tag_type<'a, S: SignTableHandle>(
    tag: TagStruct,
    closed: bool,
    expr: &mut OutDataLoader<'a, S>,
    pos: (usize, usize),
) -> Result<LoadStatus<LiteralTag<'a>, LexicalType>, LoadErr> {
    let tag_name = tag.get_name();
    match tag_name {
        "text" => {
            if !closed {
                //read body
            }

            todo!()
        }
        "sign" => {
            if tag.chcek_attr_exist("s") {
                let s = tag.get("s").unwrap().0;
                let repeat = tag
                    .get("repeat")
                    .and_then(|f| Some(f.0))
                    .and_then(|s| -> Option<u32> { s.parse().ok() })
                    .or(Some(1))
                    .unwrap();
                let sign = Sign {
                    sign: s,
                    repeat: repeat,
                };
                Ok(LoadStatus::ok(LiteralTag::Sign(sign)))
            } else {
                Err(LoadErr::unexpect("Tag Attr `s`", "Nil", pos))
            }
        }
        "endl" => Ok(LoadStatus::ok(LiteralTag::Endl)),
        _ => todo!(),
    }
}
