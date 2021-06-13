use std::collections::HashMap;

use crate::lib::anaylze::lexical::PreviewableIter;

use super::{Tag, TagAttr, TagStruct};

impl TagStruct {
    pub fn new(name: String, attrs: HashMap<String, TagAttr>) -> Self {
        TagStruct { name, attrs }
    }
}

impl TagAttr {
    pub fn get_raw(&self)->&str{
        &self.0
    }
    pub fn get_iter<'a>(&'a self)->PreviewableIter<'a>{
        self.iter()
    }
}