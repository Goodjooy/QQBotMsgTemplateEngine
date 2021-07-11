
use std::{collections::HashMap};


use super::{Sign, SignTableHandle};

struct SignTable {
    table: HashMap<String, Sign>,
    parent: Option<Box<SignTable>>,
}

impl SignTableHandle for SignTable {
    fn check_exist(&self, key: &str) -> bool {
        let parent=&self.parent;
        self.table.contains_key(key) || 
        //or in parent sign table
        match parent{
            Some(pt) => pt.check_exist(key),
            None => false,
        }
    }

    fn get_sign(&self, key: &str) -> Option<&Sign> {
        self.table.get(key)
        .or_else(
            || {
               let parent= &self.parent;
               match parent {
                Some(p) => {
                    p.get_sign(key)
                },
                None => None,
            } 
        })
    }
    /// child can edit parent value and self value
    
    fn get_mut_sign(&mut self, key: &str) -> Option<&mut Sign> {
        match self.table.get_mut(key){
            None => {
                let parent=&mut self.parent;
                match parent {
                    Some(ok) => {ok.get_mut_sign(key) },
                    None => None,
                }
            },
            ok=>ok,
        }
    }

    fn new_sign(&mut self, key: &str, value: Sign) -> Option<()> {
        if self.check_exist(key) {
            None
        } else {
            self.table.insert(key.to_string(), value);
            Some(())
        }
    }
}

impl SignTable {
    pub fn new_root()->Self{
        SignTable{
            table:HashMap::new(),
            parent:None
        }
    }
    pub fn new_child(parent:SignTable)->Self{
        SignTable{
            table:HashMap::new(),
            parent:Some(Box::new(parent))
        }
    }

    pub fn leave(self)->Option<Self>{
        self.parent.and_then(|f|Some(*f))
    }

    pub fn enter(self)->Self{
        Self::new_child(self)
    }
}