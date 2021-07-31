use crate::anaylze::{SignTableHandle, lexical::tag::{TagAttr, TagStruct}, syntax::LoadErr};
use super::{structs::CmpMod, util::load_express};

impl CmpMod {
    pub fn new<S:SignTableHandle>(ty:&str,data:&TagStruct,pos:(usize,usize),sign_table:&mut S)->Result<Self,LoadErr>{
        let ty=ty.to_lowercase();
        let v=ty.as_str();
        match v {
            "eq"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Eq(left,right))
            },
            "neq"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Neq(left,right))
            },
            "gt"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Gt(left,right))
            },
            "gte"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Gte(left,right))
            },
            "lt"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Lt(left,right))
            },
            "lte"=>{
                let left=load_express(data, "left", pos, sign_table)?;
                let right=load_express(data, "right", pos, sign_table)?;
                Ok(Self::Lte(left,right))
            },
            "boolt"=>{
                let value=load_express(data, "value", pos, sign_table)?;
                Ok(Self::BoolT(value))
            },"boolf"=>{
                let value=load_express(data, "value", pos, sign_table)?;
                Ok(Self::BoolF(value))
            }
            _=>Err(LoadErr::unexpect("Cmp Key Word", ty, pos))
        }
    }
}