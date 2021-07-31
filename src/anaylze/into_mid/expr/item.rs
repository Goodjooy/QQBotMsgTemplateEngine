use crate::anaylze::into_mid::expr::OpQuate;
use crate::anaylze::syntax::expr::{Item, SubItem};
use crate::mid_output::{IntoMid, MidData, SignIdGenerator};

use super::IntoOpQuate;

impl IntoOpQuate for SubItem {
    fn into_op(&self) -> Vec<super::OpQuate> {
        let mut res = Vec::new();
        match self {
            SubItem::Multiple(l, r) => {
                let l = &mut l.into_op();
                let r = &mut r.into_op();

                res.append(l);
                res.push(OpQuate::Mul);
                res.append(r);
            }
            SubItem::Division(l, r) => {
                let l = &mut l.into_op();
                let r = &mut r.into_op();

                res.append(l);
                res.push(OpQuate::Div);
                res.append(r);
            },
            SubItem::Nil => {}
        };
        res
    }
}

impl IntoOpQuate for Item {
    fn into_op(&self) -> Vec<super::OpQuate> {
        let r = &mut self.0.into_op();
        let l = &mut self.1.into_op();

        let mut res = vec![];
        res.append(r);
        res.append(l);
        res
    }
}
