use std::collections::HashMap;
use mockall::*;

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
    HashCache {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_get_clear_cache() {
        let mut cache = new_hash_cache();

        assert_eq!(cache.get("test"), None);

        cache.insert("test".to_string(), "value".to_string());

        assert_eq!(cache.get("test"), Some("value".to_string()));

        cache.clear();

        assert_eq!(cache.get("test"), None);
    }
}
