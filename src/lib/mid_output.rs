use std::{collections::HashMap, fmt::format};



enum MidData {
    Text(String),
    FormatText(String,Vec<TempValue>),
    Image(Image),
    At{target:u64},

    IfTrueMove(TempValue,MoveOffset),
    IfFalseMove(TempValue,MoveOffset),

    SetTemp(TempValue),
    Add(String,TempValue,TempValue),
    Sub(String,TempValue,TempValue),
    Mul(String,TempValue,TempValue),
    Div(String,TempValue,TempValue),
}

struct Image{
    id:Option<String>,
    url:Option<String>,
    path:Option<String>,
    base64:Option<String>
}

enum TempValue {
    Int(i64),
    Str(String),
    Bool(bool),
    Sign(String)
}

type MoveOffset=isize;


pub trait IntoMid {
    fn into_mid(self,id_generator:&mut SignIdGenerator)->Vec<MidData>;
}


pub struct SignIdGenerator(usize);
impl SignIdGenerator {
    pub fn new()->Self{
        Self(0)
    }
    pub fn next_id(&mut self)->String{
        self.0+=1;
       format!("t_{}", self.0)
    }
}

pub struct MidSignTable(HashMap<usize,TempValue>,usize) ;
impl MidSignTable {
    pub fn new()->Self{
        MidSignTable(HashMap::new(),0)
    }

    pub fn set_sign(&mut self,value:TempValue)->usize{
        self.0.insert(self.1, value);
        self.1+=1;
        self.1
    }

    pub fn get_sign(&self,id:usize)->Option<&TempValue>{
        self.0.get(&id)
    }


}