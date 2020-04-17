use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Cache {
    refresh_time: u64,
    cache: Arc<Mutex<HashMap<Vec<str>, Vec<String>>>>
}

impl Cache {
    pub fn new(refresh_time: u64) -> Cache {
        Cache{
            refresh_time,
            cache: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn get_vec(&self, key: &Vec<str>) -> Option<&Vec<String>> {
        self.cache.lock().unwrap().get(key)
    }

    pub fn get(&self, key: &Vec<str>) -> Option<String> {
        match self.cache.lock().unwrap().get(key) {
            Some(val) => Some(val[0].to_string()),
            None => None,
        }
    }

    pub fn insert_vec(&mut self, key: &Vec<str>, value: Vec<String>) {
        self.cache.lock().unwrap().insert(key.into_vec(), value);
    }

    pub fn insert(&mut self, key: &Vec<str>, value: &str) {
        self.cache.lock().unwrap().insert(key.into_vec(), vec![value.to_string()]);
    }
}