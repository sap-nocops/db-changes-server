use std::collections::HashMap;

pub struct Cache {
    cache: HashMap<Vec<String>, Vec<String>>
}

impl Cache {
    pub fn new() -> Cache {
        Cache{
            cache: HashMap::new()
        }
    }

    pub fn get_vec(&self, key: &Vec<String>) -> Option<Vec<String>> {
        match self.cache.get(key) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }

    pub fn get(&self, key: &Vec<String>) -> Option<String> {
        match self.cache.get(key) {
            Some(val) => Some(val[0].to_string()),
            None => None,
        }
    }

    pub fn insert_vec(&mut self, key: Vec<String>, value: Vec<String>) {
        self.cache.insert(key, value);
    }

    pub fn insert(&mut self, key: Vec<String>, value: String) {
        self.cache.insert(key, vec![value]);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        println!("cache cleared")
    }
}
