use snafu::{ResultExt, Snafu};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Unable to open file from {}: {}", path.display(), source))]
    OpenLogFile {
        source: std::io::Error,
        path: PathBuf,
    },
}

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

    pub fn open(path: &Path) -> Result<KvStore> {
        Err("unimplemented".to_string())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.store.insert(key, val);

        Err("unimplemented".to_string())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);

        Err("noimpl".to_string())
    }
}
