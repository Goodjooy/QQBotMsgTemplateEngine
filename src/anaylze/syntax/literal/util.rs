use crate::anaylze::{
    lexical::{
        tag::{
            Tag::{CloseTag, FullTag, StartTag},
            TagAttr, TagStruct,
        },
        LexicalType,
    },
    syntax::{expr::Expression, literal::ExprIter, LoadErr, SyntaxLoadNext},
    SignTableHandle,
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

pub fn load_express<S: SignTableHandle>(
    tag: &TagStruct,
    key: &str,
    pos: (usize, usize),
    sign_table: &mut S,
) -> Result<Expression, LoadErr> {
    let target_literal = load_attr(tag, key, pos)?;
    let expr_iter = target_literal.get_iter();
    let mut expr_iter = ExprIter::new(sign_table, expr_iter);
    let expr_last = expr_iter.next().ok_or(LoadErr::IterEnd)?;
    let target_expr = Expression::load_next(expr_last, &mut expr_iter)?
        .ok_or_else(|f| LoadErr::unexpect("Expression", f, expr_iter.get_postion()));

    target_expr
}

pub fn load_attr(tag: &TagStruct, key: &str, pos: (usize, usize)) -> Result<TagAttr, LoadErr> {
    tag.get(key)
        .ok_or(LoadErr::attr_not_found(key, tag.get_name(), pos))
}
pub fn check_end_tag(
    ty: &LexicalType,
    except_name: &str,
    pos: (usize, usize),
) -> Result<(), LoadErr> {
    match ty {
        LexicalType::Tag(t) => {
            if let CloseTag(s) = t {
                if s == except_name {
                    Ok(())
                } else {
                    Err(LoadErr::unexpect(
                        &format!("TagName: {}", except_name),
                        s,
                        pos,
                    ))
                }
            } else {
                Err(LoadErr::unexpect("End Tag", t, pos))
            }
        }
        _ => Err(LoadErr::unexpect("Tag", ty, pos)),
    }
}
