use std::collections::HashMap;

use crate::lib::anaylze::lexical::PreviewableIter;

use super::{TagAttr, TagStruct};

impl TagStruct {
    pub fn new(name: String, attrs: HashMap<String, TagAttr>) -> Self {
        TagStruct { name, attrs }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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
}

impl TagAttr {
    pub fn get_raw(&self) -> &str {
        &self.0
    }
    pub fn get_iter<'a>(&'a self) -> PreviewableIter<'a> {
        self.iter()
    }
}
