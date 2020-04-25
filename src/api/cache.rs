use std::collections::HashMap;
use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn insert(&mut self, key: String, value: String);
    fn clear(&mut self);
}

pub struct HashCache {
    cache: HashMap<String, String>
}

pub fn new_hash_cache() -> HashCache {
    HashCache{
        cache: HashMap::new()
    }
}

impl Cache for HashCache {
    fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).cloned()
    }

    fn insert(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
    }

    fn clear(&mut self) {
        self.cache.clear();
        println!("cache cleared")
    }
}
