use crate::anaylze::syntax::expr::Expression;
use crate::anaylze::syntax::expr::Factor::{Digit, SubExpr, Var};
use crate::anaylze::Value::{Int, List, Str, UnSet};
use crate::mid_output::{IntoMid, MidData, SignIdGenerator, TempValue};

use super::IntoOpQuate;

impl IntoOpQuate for Expression {
    fn into_op(&self) -> Vec<super::OpQuate> {
        match self {
            Expression::Caculate(c) => c.into_op(),
            Expression::Literal(_) => vec![],
        }
    }
}

impl IntoMid for Expression {
    fn into_mid(&self, id_generator: &mut SignIdGenerator) -> Vec<MidData> {
        let res = self.into_op();
        if res.len() > 0 {
            res.into_mid(id_generator)
        } else {
            match self {
                Expression::Caculate(c) => {
                    let l = &c.0;
                    let l = &l.0;
                    match l {
                        Digit(i) => {
                            vec![MidData::SetTemp(id_generator.next_id(), TempValue::Int(*i))]
                        }
                        SubExpr(s) => s.into_mid(id_generator),
                        Var(v) => {
                            let v = &v.0;
                            match &v.value {
                                UnSet(n) => vec![MidData::SetTemp(
                                    id_generator.next_id(),
                                    TempValue::Sign(n.clone()),
                                )],
                                Int(i) => vec![MidData::SetTemp(
                                    id_generator.next_id(),
                                    TempValue::Int(*i),
                                )],
                                Str(s) => vec![MidData::SetTemp(
                                    id_generator.next_id(),
                                    TempValue::Str(s.clone()),
                                )],
                                List(l) => vec![MidData::SetTemp(
                                    id_generator.next_id(),
                                    TempValue::List(
                                        l.clone().into_iter().map(|f| f.into_temp()).collect(),
                                    ),
                                )],
                            }
                        }
                    }
                }
                Expression::Literal(l) => vec![MidData::SetTemp(
                    id_generator.next_id(),
                    TempValue::Str(l.0.clone()),
                )],
            }
        }
    }
}
