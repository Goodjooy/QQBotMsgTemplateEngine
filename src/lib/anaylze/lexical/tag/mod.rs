use std::{collections::HashMap, str::Chars};

mod iter;
mod tag;
mod tag_loader;

type TagCheckFn = Fn(&str) -> bool;

#[derive(Debug, PartialEq)]
pub enum Tag {
    FullTag(TagStruct),
    StartTag(TagStruct),
    CloseTag(String),
}
#[derive(Debug, PartialEq)]
struct TagStruct {
    name: String,
    attrs: HashMap<String, TagAttr>,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TagAttr(String);
