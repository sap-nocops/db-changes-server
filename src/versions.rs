use std::fs;

pub struct Versions {
    pub apps_folder: String,
}

impl Versions {
    pub fn list(&self, app_name: String) -> Vec<String> {
        let dir = format!("{}/{}", self.apps_folder, app_name);
        let error_msg = format!("cannot read {}", dir);
        let paths = fs::read_dir(dir).expect(error_msg.as_str());
        let mut versions = Vec::new();
        for path in paths {
            match path.unwrap().file_name().into_string() {
                Ok(x) => versions.push(x),
                Err(_e) => continue,
            }
        }
        versions
    }
}
