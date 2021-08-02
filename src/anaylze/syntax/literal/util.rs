

use crate::anaylze::{
    lexical::{
        tag::{
            Tag::{CloseTag, FullTag, StartTag},
            TagAttr, TagStruct,
        },
        LexicalType, OutDataLoader,
    },
    syntax::{expr::Expression, literal::ExprIter, LoadErr, SyntaxLoadNext},
    SignTableHandle,
};

use super::{Items, TagInfo};

pub fn check_tag_match<'a, T: TagInfo>(lt: &'a LexicalType) -> Option<&'a TagStruct> {
    check_tag_name(lt, T::tag_name(), T::accept_full())
}
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

pub fn load_body<'a, S: SignTableHandle>(
    last: LexicalType,
    expr: &mut OutDataLoader<'a, S>,
    tag_name:&str
) -> Result<Option<Box<Items>>, LoadErr> {
    //body
    let body = {
        if check_end_tag(&last, tag_name, expr.get_postion()).is_err() {
            let res = Items::load_next(last, expr)?
                .ok_or_else(|f| LoadErr::unexpect("Item OR Empty", f, expr.get_postion()))?;
            // check end
            let end_tag = expr.next().ok_or(LoadErr::IterEnd)?;
            check_end_tag(&end_tag, tag_name, expr.get_postion())?;
            Some(res)
        } else {
            // close checked
            None
        }
    }
    .and_then(|body| Some(Box::new(body)));
    Ok(body)
}
