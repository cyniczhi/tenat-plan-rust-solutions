use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { store: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Option<String> {
        if let Some(v) = self.store.get(&key) {
            return Some(v.clone());
        } else {
            return None;
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
