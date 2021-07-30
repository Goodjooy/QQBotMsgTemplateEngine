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
            .and_then(|f| if f.contains_key(key) { None } else { Some(f) })?
            .insert(key.to_string(), value);
        Some(())
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

#[cfg(test)]
mod test {
    use crate::lib::anaylze::{Value, Var};

    use super::*;

    #[test]
    fn test_get_value() {
        let mut table = SignTable::new_root();
        let res1 = table.new_sign("test1", Sign::Var(Var::new("test1", Value::Int(11))));
        let res2 = table.new_sign("test2", Sign::Var(Var::new("test2", Value::Int(111))));

        assert_eq!(res1, Some(()));
        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11))))
        );

        assert_eq!(res2, Some(()));
        assert_eq!(
            table.get_sign("test2"),
            Some(&Sign::Var(Var::new("test2", Value::Int(111))))
        );

        assert_eq!(table.depath,1);
    }

    #[test]
    fn test_get_value_from_parent() {
        let mut table = SignTable::new_root();
        let res1 = table.new_sign("test1", Sign::Var(Var::new("test1", Value::Int(11))));

        table.enter();

        let res2 = table.new_sign(
            "test1",
            Sign::Var(Var::new("test1", Value::Str(String::from("aaa")))),
        );

        assert_eq!(res1, Some(()));
        assert_eq!(res2, Some(()));

        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new(
                "test1",
                Value::Str(String::from("aaa"))
            )))
        );
        assert_eq!(table.depath,2);

        table.leave();
        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11))))
        );

        assert_eq!(table.depath,1);
    }

    #[test]
    fn test_get_value_only_in_parent() {
        let mut table = SignTable::new_root();
        let res1 = table.new_sign("test1", Sign::Var(Var::new("test1", Value::Int(11))));

        table.enter();

        assert_eq!(res1, Some(()));

        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11))))
        );
        assert_eq!(table.depath,2);
        table.leave();
        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11))))
        );
        assert_eq!(table.depath,1);
    }

    #[test]
    fn test_edit_value_in_self_level() {
        let mut table = SignTable::new_root();
        let res1 = table.new_sign("test1", Sign::Var(Var::new("test1", Value::Int(11))));

        assert_eq!(res1, Some(()));
        assert_eq!(table.depath,1);

        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11))))
        );

        let value= table.get_mut_sign("test1").unwrap();

        if value.is_value(){
            let value=value.into_value_mut().unwrap();
            if let Value::Int(i)=value{
                *i+=11;
            }
        }

        assert_eq!(
            table.get_sign("test1"),
            Some(&Sign::Var(Var::new("test1", Value::Int(11+11))))
        );


        table.enter();


    }
}
