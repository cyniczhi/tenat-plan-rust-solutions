use snafu::{ResultExt, Snafu};
use std::{collections::HashMap, fs, path::Path};

// pub struct Error(InnerError);

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to open file from : {}", source))]
    OpenLogFile { source: std::io::Error },
}

type BTreeMap<K = String, V = String> = std::collections::BTreeMap<K, V>;

/// This is an implementation of in-memory k-v store
pub struct KvStore {
    sstable_: BTreeMap,
    db_file_: std::fs::File,

    index_: std::collections::HashMap<String, u64>,
    index_file_: std::fs::File,
}

impl Default for KvStore {
    fn default() -> KvStore {
        KvStore::new()
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            // curr_sstable_: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        fs::File::open(path).context(OpenLogFileSnafu {})?;

        Ok(KvStore::new())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Ok(self.curr_sstable_.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        // self.curr_sstable_.insert(key, val);

        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        // self.curr_sstable_.remove(&key);

        Ok(())
    }
}
