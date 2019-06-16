use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::collections::hash_map::RandomState;

pub mod hardware;

pub struct KeystrokeStorage {
    storage: HashMap<&'static str, u32, RandomState>
}

impl KeystrokeStorage {
    pub fn new() -> KeystrokeStorage {
        KeystrokeStorage { storage: HashMap::new() }
    }
    pub fn insert(&mut self, key: &'static str) {
        let value = *self.storage.get(&key).unwrap_or(&0);
        self.storage.insert(key, value + 1);
    }
    pub fn complete_to_string(&mut self, d: &Duration) -> String {
        // after 2100?
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Cannot compute time").as_secs() as u32;
        self.storage.insert("__time__", time);

        self.storage.insert("__duration__", d.as_secs() as u32);

        let result = serde_json::to_string(&self.storage).expect("Cannot format json");
        self.storage.clear();
        result
    }
}

