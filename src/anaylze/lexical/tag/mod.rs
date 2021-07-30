use std::{collections::HashMap};





mod iter;
mod tag;
mod tag_loader;


#[derive(Debug,Clone,PartialEq, )]

pub enum Tag {
    FullTag(TagStruct),
    StartTag(TagStruct),
    CloseTag(String),
}
#[derive(Debug,Clone,PartialEq, )]

pub struct TagStruct {
    name: String,
    attrs: HashMap<String, TagAttr>,
}
#[derive(Debug,Clone,PartialEq, )]

pub struct TagAttr(pub String);


