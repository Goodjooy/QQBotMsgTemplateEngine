use std::{collections::HashMap, fmt::format};

pub enum MidData {
    Text(String),
    FormatText(String, Vec<TempValue>),
    Image(Image),
    At { target: u64 },

    IfTrueMove(TempValue, MoveOffset),
    IfFalseMove(TempValue, MoveOffset),

    SetTemp(String, TempValue),
    Add(String, TempValue, TempValue),
    Sub(String, TempValue, TempValue),
    Mul(String, TempValue, TempValue),
    Div(String, TempValue, TempValue),
}

impl MidData {
    pub fn get_sign(&self) -> Option<&String> {
        match self {
            Self::SetTemp(s, _)
            | Self::Add(s, _, _)
            | Self::Sub(s, _, _)
            | Self::Mul(s, _, _)
            | Self::Div(s, _, _) => Some(s),
            _ => None,
        }
    }
}

pub struct Image {
    id: Option<String>,
    url: Option<String>,
    path: Option<String>,
    base64: Option<String>,
}

pub enum TempValue {
    Int(i64),
    Str(String),
    Bool(bool),
    List(Vec<TempValue>),
    Sign(String),
}

type MoveOffset = isize;

pub trait IntoMid {
    fn into_mid(&self, id_generator: &mut SignIdGenerator) -> Vec<MidData>;
}

pub struct SignIdGenerator(usize);
impl SignIdGenerator {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn next_id(&mut self) -> String {
        self.0 += 1;
        format!("t_{}", self.0)
    }
}

pub struct MidSignTable(HashMap<usize, TempValue>, usize);
impl MidSignTable {
    pub fn new() -> Self {
        MidSignTable(HashMap::new(), 0)
    }

    pub fn set_sign(&mut self, value: TempValue) -> usize {
        self.0.insert(self.1, value);
        self.1 += 1;
        self.1
    }

    pub fn get_sign(&self, id: usize) -> Option<&TempValue> {
        self.0.get(&id)
    }
}
