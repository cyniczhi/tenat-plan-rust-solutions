use std::collections::HashMap;

/// This is an implementation of in-memory k-v store
pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> KvStore {
        KvStore::new()
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
