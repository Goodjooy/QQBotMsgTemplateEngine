use crate::lib::anaylze::syntax::expr::Factor;


mod factor;
mod item;

enum OpQuate {
    F(Factor),
    Add,
    Sub,
    Mul,
    Div
}