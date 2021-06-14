use crate::lib::anaylze::lexical::util::{check_next_sign, clear_space};
use std::collections::HashMap;

use crate::lib::anaylze::lexical::PreviewableIter;

use super::{Tag, TagAttr, TagStruct};

impl Tag {
    pub fn load_next(data: &mut PreviewableIter) -> Option<Tag> {
        clear_space(data)?;
        //asumme that commant and space hasbeen removed
        let start_sign = data.preview()?;
        if start_sign != '<' {
            None
        } else {
            data.next()?;
            if Self::clear_comment(data)? {
                clear_space(data)?;
                if !check_next_sign('<', true, data)? {
                    return None;
                }
            }
            let (tag, is_closed) = Self::load_tag(data)?;
            if is_closed {
                let close = data.preview()?;
                if close == '>' {
                    data.next()?;
                    Some(Tag::CloseTag(tag))
                } else {
                    None
                }
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
                    if (d.is_whitespace() || d == '/' || d == '>') && tag.len() > 0 {
                        break Some((tag, closer));
                    } else {
                        if d == '/' && tag.len() == 0 {
                            data.next()?;
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
    fn clear_comment(data: &mut PreviewableIter) -> Option<bool> {
        if check_next_sign('?', true, data)? {
            loop {
                match data.preview() {
                    Some(c) => {
                        if c != '?' {
                            data.next()?;
                        } else {
                            data.next()?;
                            if check_next_sign('>', true, data)? {
                                break Some(true);
                            }
                        }
                    }
                    None => break None,
                }
            }
        } else {
            Some(false)
        }
    }
}

#[cfg(test)]
mod tag {
    use super::*;
    #[test]
    fn test_clear_cmd() {
        let mut iter = PreviewableIter::new(r#"?/if mod="eq"?><if>"#);

        let res = Tag::clear_comment(&mut iter).unwrap();

        assert_eq!(iter.next().unwrap(), '<')
    }
    #[test]
    fn test_load_tag() {
        let mut iter = PreviewableIter::new(r#"/if mod="eq""#);

        let res = Tag::load_tag(&mut iter).unwrap();

        assert_eq!(res.1, true);
        assert_eq!(res.0, "if");
    }
    #[test]
    fn test_load_attr_name() {
        let mut iter = PreviewableIter::new(r#"attr1="atat"#);

        let res = Tag::load_attr_name(&mut iter).unwrap();

        assert_eq!(res, "attr1");
    }

    #[test]
    fn test_load_attr_value() {
        let mut iter = PreviewableIter::new(r#""atat"#);

        let res = Tag::load_attr_value(&mut iter).unwrap();

        assert_eq!(res.get_raw(), "atat");
    }

    #[test]
    fn test_load_attrs() {
        let mut iter = PreviewableIter::new(r#"attr1="atat" attr2="aaa" >"#);

        let res = Tag::load_attrs(&mut iter).unwrap();

        assert_eq!(res.get("attr1").unwrap().get_raw(), "atat".to_string());
        assert_eq!(res.get("attr2").unwrap().get_raw(), "aaa".to_string());
        assert_eq!(iter.next().unwrap(), '>')
    }

    #[test]
    fn test_load_next_tag() {
        let mut iter = PreviewableIter::new(
            r#"<img file="/var/pic/..."/>
            <?<abb a="aaa">?>
        <if mod="eq" left="value_name" right="'value'"></if>"#,
        );

        let res = Tag::load_next(&mut iter).unwrap();

        if let Tag::FullTag(s) = res {
            assert_eq!(s.name, "img");
            assert_eq!(s.attrs.get("file").unwrap().get_raw(), "/var/pic/...");
        } else {
            panic!("Bad type")
        }
        assert_eq!(iter.preview(), Some('\n'));

        let res = Tag::load_next(&mut iter).unwrap();
        if let Tag::StartTag(s) = res {
            assert_eq!(s.name, "if");
            assert_eq!(s.attrs.get("mod").unwrap().get_raw(), "eq");
            assert_eq!(s.attrs.get("left").unwrap().get_raw(), "value_name");
            assert_eq!(s.attrs.get("right").unwrap().get_raw(), "'value'");
        } else {
            panic!("Bad type")
        }
        assert_eq!(iter.preview(), Some('<'));

        let res = Tag::load_next(&mut iter).unwrap();
        if let Tag::CloseTag(s) = res {
            assert_eq!(s, "if");
        } else {
            panic!("Bad type")
        }

        assert_eq!(iter.next(), None);
    }
}
