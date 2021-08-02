use crate::anaylze::{SignTableHandle, lexical::{
        tag::{TagAttr, TagStruct},
        OutDataLoader,
    }, syntax::{LoadErr, literal::{structs::ValueOperate, util::load_express}}};

mod var;

impl ValueOperate {
    pub fn new<'a, S: SignTableHandle>(
        tag: &TagStruct,
        expr: &mut OutDataLoader<'a, S>,
    ) -> Result<Self, LoadErr> {
        let op_mod = {
            let da = tag.get("mod").unwrap_or(TagAttr(String::from("print")));
            da.get_raw_owner()
        };
        match op_mod.as_str() {
            "new" => Ok(ValueOperate::New),
            "need"=>Ok(ValueOperate::Need),
            "assign" => {
                let ex = load_express(tag, "value", expr.get_postion(), expr.get_sign_table())?;
                Ok(ValueOperate::Assign(ex))
            }
            "print" => {
                let format = {
                    let da = tag.get("format").unwrap_or(TagAttr(String::from("{}")));
                    da.get_raw_owner()
                };
                Ok(ValueOperate::Print(format))
            }
            "println" => {
                let format = {
                    let da = tag.get("format").unwrap_or(TagAttr(String::from("{}")));
                    da.get_raw_owner()
                };
                Ok(ValueOperate::Println(format))
            }
            m => Err(LoadErr::unexpect(
                "Value Operate Sign",
                m,
                expr.get_postion(),
            )),
        }
    }
}
