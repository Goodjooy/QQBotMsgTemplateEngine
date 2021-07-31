use crate::anaylze::into_mid::expr::OpQuate;
use crate::anaylze::syntax::expr::Factor;
use crate::mid_output::IntoMid;
use crate::mid_output::{MidData, TempValue};

use super::IntoOpQuate;

impl IntoOpQuate for Factor {
    fn into_op(&self) -> Vec<super::OpQuate> {
        match self {
            Factor::Digit(d) => vec![OpQuate::F(*d)],
            Factor::SubExpr(e) => e.into_op(),
            Factor::Var(v) => {
                let v = &v.0;
                let name = &v.name;
                let res = match v.value {
                    crate::anaylze::Value::UnSet(_) => {
                        vec![OpQuate::Sign(name.clone())]
                    }
                    crate::anaylze::Value::Int(i) => {
                        vec![OpQuate::F(i)]
                    }
                    crate::anaylze::Value::Str(_) | crate::anaylze::Value::List(_) => vec![],
                };
                res
            }
        }
    }
}
