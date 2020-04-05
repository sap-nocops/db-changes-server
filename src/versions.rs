use std::fs;

pub struct Versions {
    pub appsFolder: String,
}

impl Versions {
    pub fn list(&self, appName: String) -> Vec<String> {
        let paths = fs::read_dir(format!("{}/{}", self.appsFolder, appName)).unwrap();
        let mut versions = Vec::new();
        for path in paths {
            versions.push(path.unwrap().path().display());
        }
        versions
    }
}
