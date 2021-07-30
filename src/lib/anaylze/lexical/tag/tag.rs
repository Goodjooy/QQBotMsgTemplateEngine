use std::fmt::Display;
use std::collections::HashMap;

use crate::lib::anaylze::lexical::{PreviewableIter, tag::Tag};

use super::{TagAttr, TagStruct};

impl TagStruct {
    pub fn new(name: String, attrs: HashMap<String, TagAttr>) -> Self {
        TagStruct { name, attrs }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_attrs(&self)->&HashMap<String,TagAttr>{
&self.attrs
    }

    pub fn chcek_attr_exist(&self, k: &str) -> bool {
        self.attrs.contains_key(k)
    }

    pub fn check_attr_all_exist(&self, ks: &[&str]) -> bool {
        let t: Vec<_> = ks.iter().filter(|f| !self.chcek_attr_exist(f)).collect();

        t.len() == 0
    }

    pub fn get(&self, key: &str) -> Option<TagAttr> {
        self.attrs.get(key).and_then(|f| Some(f.clone()))
    }

    pub fn get_default(&self,key:&str,default:&str)->TagAttr{
        self.attrs.get(key).and_then(|f| Some(f.clone())).unwrap_or(TagAttr(default.to_string()))
    }
}

impl TagAttr {
    pub fn get_raw(&self) -> &str {
        &self.0
    }
    pub fn get_raw_owner(self)->String{
        self.0
    }
    pub fn get_iter<'a>(&'a self) -> PreviewableIter<'a> {
        self.iter()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::FullTag(ft) => write!(f, "ClosedTag: {}", ft),
            Tag::StartTag(ft) => write!(f, "OpendTag: {}", ft),
            Tag::CloseTag(s) => write!(f, "ClosingTag: {}", s),
        }
    }
}

impl Display for TagStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}", self.get_name())?;
        let attrs = self
            .get_attrs()
            .iter()
            .map(|d| format!("{}: {}", d.0, d.1))
            .reduce(|f, d| format!("{}, {}", f, d))
            .and_then(|f| Some(format!("{{ {} }}", f)))
            .unwrap();

        write!(f,", attr: {}",attrs)
    }
}

impl Display for TagAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}