use crate::lib::mid_output::{IntoMid, MidData, SignIdGenerator};
use crate::lib::anaylze::syntax::expr::SubItem;

impl IntoMid for SubItem {
    fn into_mid(self,id_generator:&mut SignIdGenerator)->Vec<MidData> {
        let mut res=Vec::new();

        match self {
            SubItem::Multiple(f, s) => {
                let fac=f.into_mid(id_generator);
                let fac_res=fac.last().and_then(|f|f.get_sign());
                todo!()

            },
            SubItem::Division(f, s) => todo!(),
            SubItem::Nil => {},
        }

        res
    }
}