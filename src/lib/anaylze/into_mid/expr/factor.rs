use crate::lib::mid_output::{MidData, TempValue};
use crate::lib::anaylze::syntax::expr::Factor;
use crate::lib::mid_output::IntoMid;


impl IntoMid for Factor {
    fn into_mid(self,id_generator:&mut crate::lib::mid_output::SignIdGenerator)->Vec<MidData> {
        let mut res=Vec::new();
        match self {
            Factor::Digit(d) =>{
                let name=id_generator.next_id();
                res.push(MidData::SetTemp(name,TempValue::Int(d)))
            },
            Factor::SubExpr(e) => todo!(),
            Factor::Var(s) => {
                let sign=s.0;
                let var=sign.value;
                res.push(MidData::SetTemp(id_generator.next_id(),var.into_temp()))
            },
        };

        res
    }
}