
use crate::anaylze::lexical::PreviewableIter;

use super::{TagAttr, };


impl TagAttr {
     pub fn iter<'a>(&'a self) -> PreviewableIter<'a> {
        PreviewableIter::new(&self.0[..])
    }
}
