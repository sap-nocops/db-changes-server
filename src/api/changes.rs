use std::fs::File;
use std::io::prelude::*;

pub struct Changes {
    pub apps_path: String,
}

impl Changes {
    pub fn get(&self, app_name: String, db_version: String) -> String {
        let error_msg = "cannot get changes";
        let mut file = File::open(format!("{}/{}/{}", self.apps_path, app_name, db_version)).expect(error_msg);
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(error_msg);
        contents
    }
}
