use crate::anaylze::{
    into_mid::expr::OpQuate,
    syntax::expr::{Caculate, SubCaculate},
};

use super::IntoOpQuate;

impl IntoOpQuate for Caculate {
    fn into_op(&self) -> Vec<super::OpQuate> {
        let l = &mut self.0.into_op();
        let r = &mut self.1.into_op();

        let mut res = Vec::new();
        res.append(l);
        res.append(r);
        res
    }
}

impl IntoOpQuate for SubCaculate {
    fn into_op(&self) -> Vec<super::OpQuate> {
        let mut res = Vec::new();
        match self {
            SubCaculate::Addition(l, r) => {
                let l = &mut l.into_op();
                let r = &mut r.into_op();

                res.append(l);
                res.push(OpQuate::Add);
                res.append(r);
            }
            SubCaculate::Subtraction(l, r) => {
                let l = &mut l.into_op();
                let r = &mut r.into_op();

                res.append(l);
                res.push(OpQuate::Sub);
                res.append(r);
            }
            SubCaculate::Nil => {}
        }
        res
    }
}
