use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use super::cache::Cache;

pub struct Changes<'a> {
    apps_path: String,
    cache: &'a Cache,
}

impl Changes {
    pub fn new(apps_path: &str, cache: &Cache) -> Changes {
        Changes {
            apps_path: apps_path.to_string(),
            cache,
        }
    }

    pub fn get(&self, app_name: &str, db_version: &str) -> Result<String, Error> {
        let key = vec!["changes".to_string(), app_name.to_string(), db_version.to_string()];
        match self.cache.get(&key) {
            Some(val) => val,
            None => {
                let mut file = File::open(format!("{}/{}/{}", self.apps_path, app_name, db_version))?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                self.cache.insert(&key, &contents);
                Ok(contents)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_changes_for_app() {
        let changes_api = Changes {
            apps_path: String::from("test_data")
        };

        match changes_api.get("test_app", "change") {
            Ok(val) => assert_eq!(val, String::from("INSERT INTO frattaglie (id, nome) VALUES (1, 'lampredotto');\n")),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn error_app_not_found() {
        let changes_api = Changes {
            apps_path: String::from("test_data")
        };

        match changes_api.get("non_app", "change") {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn error_db_version_not_found() {
        let changes_api = Changes {
            apps_path: String::from("test_data")
        };

        match changes_api.get("test_app", "non_change") {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }
}