use std::{collections::HashMap, ptr::read_volatile};

use crate::lib::anaylze::lexical::{clear_space, PreviewableIter};

use super::{Tag, TagAttr, TagStruct};

impl Tag {
    pub fn load_next(data: &mut PreviewableIter) -> Option<Tag> {
        //asumme that commant and space hasbeen removed
        let start_sign = data.preview()?;
        if start_sign != '<' {
            None
        } else {
            data.next()?;
            let (tag, is_closed) = Self::load_tag(data)?;
            if is_closed {
                Some(Tag::CloseTag(tag))
            } else {
                let attrs = Self::load_attrs(data)?;
                clear_space(data);
                let is_self_close = Self::check_close_sign(data)?;
                let close = data.next()?;
                if close == '>' {
                    if is_self_close {
                        Some(Tag::FullTag(TagStruct::new(tag, attrs)))
                    } else {
                        Some(Tag::StartTag(TagStruct::new(tag, attrs)))
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl Tag {
    fn load_tag(data: &mut PreviewableIter) -> Option<(String, bool)> {
        let mut tag = String::new();
        let mut closer: bool = false;
        clear_space(data);
        loop {
            match data.preview() {
                Some(d) => {
                    if d.is_whitespace() {
                        break Some((tag, closer));
                    } else {
                        if d == '/' && tag.len() == 0 {
                            closer = true
                        }
                        tag.push(data.next()?);
                    }
                }
                None => break None,
            }
        }
    }

    fn load_attrs(data: &mut PreviewableIter) -> Option<HashMap<String, TagAttr>> {
        let mut map = HashMap::new();
        loop {
            clear_space(data);
            match data.preview() {
                Some(ch) => {
                    if ch == '/' || ch == '>' || ch.is_whitespace() {
                        break Some(map);
                    }
                    let attr_name = Self::load_attr_name(data)?;
                    clear_space(data);
                    if data.preview()? == '=' {
                        data.next()?;
                    }
                    clear_space(data);
                    let attr_value = Self::load_attr_value(data)?;

                    map.insert(attr_name, attr_value);
                }
                None => break None,
            }
        }
    }

    fn load_attr_name(data: &mut PreviewableIter) -> Option<String> {
        let mut name = String::new();
        clear_space(data);
        loop {
            match data.preview() {
                Some(ch) => {
                    if ch == '=' {
                        break Some(name);
                    } else {
                        name.push(data.next()?);
                    }
                }
                None => break None,
            }
        }
    }
    fn load_attr_value(data: &mut PreviewableIter) -> Option<TagAttr> {
        let mut value: String = String::new();
        clear_space(data);

        let left_quote = data.preview()?;
        if left_quote != '"' {
            return None;
        } else {
            data.next()?;
        }

        loop {
            match data.preview() {
                Some(ch) => {
                    if ch != '"' {
                        value.push(data.next()?);
                    } else {
                        data.next();
                        break;
                    }
                }
                None => break,
            }
        }
        Some(TagAttr(value))
    }

    fn check_close_sign(data: &mut PreviewableIter) -> Option<bool> {
        clear_space(data);
        let next = data.preview()?;
        if next == '/' {
            data.next()?;
            return Some(true);
        } else if next == '>' {
            return Some(false);
        } else {
            return None;
        }
    }
}
