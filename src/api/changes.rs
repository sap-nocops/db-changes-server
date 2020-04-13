use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub struct Changes {
    pub apps_path: String,
}

impl Changes {
    pub fn get(&self, app_name: String, db_version: String) -> Result<String, Error> {
        let mut file = File::open(format!("{}/{}/{}", self.apps_path, app_name, db_version))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
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

        match changes_api.get(String::from("test_app"), String::from("change")) {
            Ok(val) => assert_eq!(val, String::from("INSERT INTO frattaglie (id, nome) VALUES (1, 'lampredotto');\n")),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn error_app_not_found() {
        let changes_api = Changes {
            apps_path: String::from("test_data")
        };

        match changes_api.get(String::from("non_app"), String::from("change")) {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn error_db_version_not_found() {
        let changes_api = Changes {
            apps_path: String::from("test_data")
        };

        match changes_api.get(String::from("test_app"), String::from("non_change")) {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }
}