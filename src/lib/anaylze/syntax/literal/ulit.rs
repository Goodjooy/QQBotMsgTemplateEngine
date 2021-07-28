use crate::lib::anaylze::lexical::{
    tag::{
        Tag::{FullTag, StartTag},
        TagStruct,
    },
    LexicalType,
};

pub fn check_tag_name<'a>(
    ty: &'a LexicalType,
    accept_name: &str,
    accept_full: bool,
) -> Option<&'a TagStruct> {
    match ty {
        LexicalType::Tag(t) => match t {
            FullTag(ft) => {
                if accept_full && ft.get_name() == accept_name {
                    Some(ft)
                } else {
                    None
                }
            }
            StartTag(st) => {
                if st.get_name() == accept_name {
                    Some(st)
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

