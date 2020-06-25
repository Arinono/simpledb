use std::collections::HashMap;

pub trait SimpleDB {
    fn set(&mut self, key: String, value: u32);
    fn get(&mut self, key: String) -> Option<&u32>;
    fn unset(&mut self, key: String);
    fn begin_transaction(&mut self);
    fn rollback(&mut self) -> Result<(), String>;
    fn commit(&mut self) -> Result<(), String>;
}

pub struct InMemoryDB {
    db: Vec<HashMap<String, u32>>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            db: vec![HashMap::new()],
        }
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {
        if let Some(last) = self.db.last_mut() {
            last.insert(key, value);
        }
    }

    fn get(&mut self, key: String) -> Option<&u32> {
        if let Some(last) = self.db.last() {
            return last.get(&key);
        } else {
            None
        }
    }

    fn unset(&mut self, key: String) {
        let last = self.db.last_mut().unwrap();
        last.remove(&key);
    }

    fn begin_transaction(&mut self) {
        let new_db = self.db.last().unwrap().clone();
        self.db.push(new_db);
    }

    fn rollback(&mut self) -> Result<(), String> {
        match self.db.pop() {
            Some(_last) => {
                if self.db.len() == 0 {
                    return Err(String::from("No transactions in progress"))
                }
                Ok(())
            },
            None => Err(String::from("No transactions in progress"))
        }
    }

    fn commit(&mut self) -> Result<(), String> {
        match self.db.pop() {
            Some(last) => {
                if self.db.len() == 0 {
                    return Err(String::from("No transactions in progress"))
                }
                self.db = vec![last];
                Ok(())
            },
            None => Err(String::from("No transactions in progress"))
        }
    }
}
