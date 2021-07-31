use crate::anaylze::syntax::expr::Factor;
use crate::mid_output::{IntoMid, MidData, SignIdGenerator, TempValue};

mod factor;
mod item;
mod expression;
mod caculate;

#[derive(Debug,PartialEq,Clone,PartialOrd)]
pub enum OpQuate {
    F(i64),
    Sign(String),
    Add,
    Sub,
    Mul,
    Div,
}

enum InerData {
    I(i64),
    Sign(String),
}

trait IntoOpQuate {
    fn into_op(&self) -> Vec<OpQuate>;
}

impl IntoMid for dyn IntoOpQuate{
    fn into_mid(&self, id_generator: &mut SignIdGenerator) -> Vec<MidData> {
       let data=self.into_op();
       data.into_mid(id_generator)
    }
}

impl IntoMid for Vec<OpQuate> {
    fn into_mid(&self, id_generator: &mut SignIdGenerator) -> Vec<MidData> {
        let iter = self.into_iter();
        let mut res = Vec::new();
        let mut stack = Vec::new();

        for da in iter {
            match da {
                OpQuate::F(f) => {
                    stack.push(InerData::new_num(*f));
                }
                OpQuate::Add => {
                    let right = stack.pop().expect("No Enought Data In Stack");
                    let left = stack.pop().expect("No Enought Data In Stack");
                    let name = id_generator.next_id();
                    stack.push(InerData::new_sign(&name));
                    let d = MidData::Add(name, left.into_temp(), right.into_temp());
                    res.push(d);
                }
                OpQuate::Sub => {
                    let right = stack.pop().expect("No Enought Data In Stack");
                    let left = stack.pop().expect("No Enought Data In Stack");
                    let name = id_generator.next_id();
                    stack.push(InerData::new_sign(&name));
                    let d = MidData::Sub(name, left.into_temp(), right.into_temp());
                    res.push(d);
                },
                OpQuate::Mul => {
                    let right = stack.pop().expect("No Enought Data In Stack");
                    let left = stack.pop().expect("No Enought Data In Stack");
                    let name = id_generator.next_id();
                    stack.push(InerData::new_sign(&name));
                    let d = MidData::Mul(name, left.into_temp(), right.into_temp());
                    res.push(d);
                },
                OpQuate::Div => {
                    let right = stack.pop().expect("No Enought Data In Stack");
                    let left = stack.pop().expect("No Enought Data In Stack");
                    let name = id_generator.next_id();
                    stack.push(InerData::new_sign(&name));
                    let d = MidData::Div(name, left.into_temp(), right.into_temp());
                    res.push(d);
                },
                OpQuate::Sign(s) =>  stack.push(InerData::new_sign(s)),
            }
        }
        res
    }
}

impl InerData {
    fn new_num(n: i64) -> Self {
        Self::I(n)
    }
    fn new_sign(name: &str) -> Self {
        Self::Sign(name.to_string())
    }
    fn into_temp(self) -> TempValue {
        match self {
            InerData::I(i) => TempValue::Int(i),
            InerData::Sign(s) => TempValue::Sign(s),
        }
    }
}
