use std::collections::HashMap;

use super::{Tag, TagAttr, TagStruct};

impl TagStruct {
    pub fn new(name: String, attrs: HashMap<String, TagAttr>) -> Self {
        TagStruct { name, attrs }
    }
}
