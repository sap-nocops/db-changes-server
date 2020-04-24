use std::collections::HashMap;
use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait Cache {
    fn new() -> dyn Cache;
    fn get(&self, key: String) -> Option<String>;
    fn insert(&mut self, key: String, value: String);
    fn clear(&mut self);
}

pub struct HashCache {
    cache: HashMap<String, String>
}

impl Cache for HashCache {
    fn new() -> HashCache {
        HashCache{
            cache: HashMap::new()
        }
    }

    fn get(&self, key: String) -> Option<String> {
        *self.cache.get(&key)
    }

    fn insert(&mut self, key: String, value: String) {
        self.cache.insert(key.to_string(), value.to_string());
    }

    fn clear(&mut self) {
        self.cache.clear();
        println!("cache cleared")
    }
}
