use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct KvStore {
    data: HashMap<String, String>
}


impl KvStore {
    pub fn new() -> Self {
        KvStore { data: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}


pub type SharedKvShare = Arc<Mutex<KvStore>>;


