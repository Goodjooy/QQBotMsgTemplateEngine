
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::{collections::HashMap};


use super::{Sign, SignTableHandle};

struct SignTable {
    table: HashMap<String, Sign>,
    parent: Option<Rc<RefCell<SignTable>>>,

}

impl SignTableHandle for SignTable {
    fn check_exist(&self, key: &str) -> bool {
        let parent=&self.parent;
        self.table.contains_key(key) || 
        //or in parent sign table
        match parent{
            Some(pt) => Rc::clone(pt).borrow_mut().check_exist(key),
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
                    let t=p.borrow_mut();
                    todo!()
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
                    Some(ok) => {todo!() },
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

    fn leave(s:Rc<RefCell<Self>>)->Option<Rc<RefCell<Self>>> {
        let b=(s).borrow_mut().parent.clone();
        return b;
    }

    fn enter(s:Rc<RefCell<Self>>)->Self {
        Self::new_child(s)
    }

    
     
}

impl SignTable {
    pub fn new_root()->Self{
        SignTable{
            table:HashMap::new(),
            parent:None,
            
        }
    }
    pub fn new_child(parent: Rc<RefCell<SignTable>>)->SignTable{
        let mut t=SignTable{
            table:HashMap::new(),
            parent:Some(parent),
        };
     
        t
    }

    
}