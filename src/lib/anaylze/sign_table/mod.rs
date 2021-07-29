use std::collections::HashMap;

use super::{Sign, SignTableHandle};

struct SignTable {
    tables: Vec<HashMap<String, Sign>>,
    depath: usize,
}

impl SignTableHandle for SignTable {
    fn check_exist(&self, key: &str) -> bool {
        let size = self.depath;
        let range = 0..size;

        for index in range.rev() {
            match self.tables.get(index) {
                Some(d) => {
                    if d.contains_key(key) {
                        return true;
                    }
                }
                None => continue,
            };
        }
        return false;
    }

    fn get_sign(&self, key: &str) -> Option<&Sign> {
        let size = self.depath;
        let range = 0..size;

        for index in range.rev() {
            match self.tables.get(index) {
                Some(d) => {
                    if d.contains_key(key) {
                        return d.get(key);
                    }
                }
                None => continue,
            };
        }
        return None;
    }
    /// child can edit parent value and self value
    fn get_mut_sign(&mut self, key: &str) -> Option<&mut Sign> {
        self.tables
            .iter_mut()
            .filter(|map| map.contains_key(key))
            .last()
            .and_then(|f| f.get_mut(key))
    }

    fn new_sign(&mut self, key: &str, value: Sign) -> Option<()> {
        self.tables
            .last_mut()
            .and_then(|f| if f.contains_key(key) { None } else { Some(f) })
            .and_then(|f| f.insert(key.to_string(), value))
            .and_then(|_| Some(()))
    }

    fn leave(&mut self) {
        self.tables.pop();
        self.depath = self.tables.len();
    }

    fn enter(&mut self) {
        self.new_child();
    }
}

impl SignTable {
    pub fn new_root() -> Self {
        SignTable {
            tables: vec![HashMap::new()],
            depath: 1,
        }
    }

    pub fn new_child(&mut self) {
        self.tables.push(HashMap::new());
        self.depath = self.tables.len();
    }
}
