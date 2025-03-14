use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub struct Cache {
    data: HashMap<String, (Vec<u8>, SystemTime)>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        if let Some((data, timestamp)) = self.data.get(key) {
            if SystemTime::now().duration_since(*timestamp).unwrap() < Duration::from_secs(600) {
                return Some(data);
            }
        }
        None
    }

    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        self.data.insert(key, (data, SystemTime::now()));
    }
}
