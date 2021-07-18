use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
#[derive(Default)]
pub struct KvStore {
    db: HashMap<String, String>,
}

impl KvStore {
    /// Create a new KvStore
    pub fn new() -> KvStore {
        KvStore { db: HashMap::new() }
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    /// Get the string value of the a string key.
    ///
    /// If the key does not exist, return `None`.
    pub fn get(&self, key: String) -> Option<String> {
        self.db.get(&key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.db.remove(&key);
    }
}
