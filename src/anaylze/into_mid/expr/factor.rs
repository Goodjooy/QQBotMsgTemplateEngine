use crate::anaylze::{
        into_mid::expr::OpQuate,
        syntax::expr::Factor,
        Value::{Bool, List, Str},
    };

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
                    Bool(_) | Str(_) | List(_) => vec![],
                };
                res
            }
        }
    }
}
#[cfg(test)]
mod test {
    use crate::anaylze::syntax::expr::{Caculate, ExprVar, Item, SubItem};
    use crate::anaylze::syntax::expr::{Expression, SubCaculate};
    use crate::anaylze::Value;
    use crate::anaylze::Var;

    use super::*;

    #[test]
    fn test_digit_into_quate() {
        let v = Factor::Digit(11);

        let res = v.into_op();

        assert_eq!(res, vec![OpQuate::F(11)])
    }

    #[test]
    fn test_sub_expr_into() {
        // 11/2*5+9-(-9)
        let v = Factor::SubExpr(Box::new(Expression::Caculate(Caculate(
            Item(
                Factor::Digit(11),
                SubItem::Division(
                    Factor::Digit(2),
                    Box::new(SubItem::Multiple(Factor::Digit(5), Box::new(SubItem::Nil))),
                ),
            ),
            SubCaculate::Addition(
                Item(Factor::Digit(9), SubItem::Nil),
                Box::new(SubCaculate::Subtraction(
                    Item(
                        Factor::Var(ExprVar(Var {
                            name: String::from("test1"),
                            value: Value::UnSet(String::from("test1")),
                        })),
                        SubItem::Nil,
                    ),
                    Box::new(SubCaculate::Nil),
                )),
            ),
        ))));

        let res = v.into_op();

        assert_eq!(
            res,
            vec![
                OpQuate::F(11),
                OpQuate::F(2),
                OpQuate::Div,
                OpQuate::F(5),
                OpQuate::Mul,
                OpQuate::F(9),
                OpQuate::Add,
                OpQuate::Sign(String::from("test1")),
                OpQuate::Sub
            ]
        )
    }
}
