use std::{collections::HashMap, str::Chars};

mod iter;
mod tag;
mod tag_loader;

type TagCheckFn = Fn(&str) -> bool;

pub enum Tag {
    FullTag(TagStruct),
    StartTag(TagStruct),
    CloseTag(String),
}

struct TagStruct {
    name: String,
    attrs: HashMap<String, TagAttr>,
}

pub struct TagAttr (String);

