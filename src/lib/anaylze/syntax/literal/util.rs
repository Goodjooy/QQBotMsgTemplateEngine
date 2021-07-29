use crate::lib::anaylze::syntax::literal::ExprIter;
use crate::lib::anaylze::syntax::SyntaxLoadNext;
use crate::lib::anaylze::SignTableHandle;
use crate::lib::anaylze::{
    lexical::{
        tag::{
            Tag::{FullTag, StartTag},
            TagStruct,
        },
        LexicalType,
    },
    syntax::{expr::Expression, LoadErr},
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
    let target_literal =
        tag.get(key)
            .ok_or(LoadErr::attr_not_found(key, tag.get_name(), pos))?;
    let expr_iter = target_literal.get_iter();
    let mut expr_iter = ExprIter::new(sign_table, expr_iter);
    let expr_last = expr_iter.next().ok_or(LoadErr::IterEnd)?;
    let target_expr = Expression::load_next(expr_last, &mut expr_iter)?
        .ok_or_else(|f| LoadErr::unexpect("Expression", f, expr_iter.get_postion()));

    target_expr
}
