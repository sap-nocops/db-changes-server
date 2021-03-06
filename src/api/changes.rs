use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait ChangesApi {
    fn get(&self, app_name: &str, db_version: &str) -> Result<String, Error>;
}

pub fn new_file_changes_api(apps_path: &str) -> FileChangesApi {
    FileChangesApi {
        apps_path: apps_path.to_string()
    }
}

pub struct FileChangesApi {
    apps_path: String
}

impl ChangesApi for FileChangesApi {
    fn get(&self, app_name: &str, db_version: &str) -> Result<String, Error> {
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
        let changes_api = FileChangesApi {
            apps_path: String::from("test_data")
        };

        match changes_api.get("test_app", "change") {
            Ok(val) => assert_eq!(val, String::from("INSERT INTO frattaglie (id, nome) VALUES (1, 'lampredotto');\n")),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn error_app_not_found() {
        let changes_api = FileChangesApi {
            apps_path: String::from("test_data")
        };

        match changes_api.get("non_app", "change") {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn error_db_version_not_found() {
        let changes_api = FileChangesApi {
            apps_path: String::from("test_data")
        };

        match changes_api.get("test_app", "non_change") {
            Ok(_val) => assert!(false),
            Err(_e) => assert!(true),
        }
    }
}
