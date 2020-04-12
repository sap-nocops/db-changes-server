use std::fs;
use rusqlite::{params, Connection};

#[derive(Debug)]
struct DbVersion {
    version: String,
}

pub struct Versions {
    pub db_path: String,
}

impl Versions {
    pub fn list(&self, app_name: String, app_version: String) -> Vec<String> {
        let error_msg = format!("cannot retrieve db versions for {} {}", app_name.clone(), app_version.clone());
        let conn = Connection::open(self.db_path.clone()).expect(error_msg.as_str());
        let mut stmt = conn.prepare("SELECT dv.version FROM apps a JOIN apps_db_versions adv
         ON a.id = adv.app_id JOIN db_versions dv ON dv.id = adv.db_id
         WHERE a.name = ? AND a.version = ?").expect(error_msg.as_str());
        let db_version_iter = stmt.query_map(params![app_name, app_version], |row| {
            Ok(DbVersion{version: row.get(0)?,})
        }).expect(error_msg.as_str());
        let mut db_versions = Vec::new();
        for db_version in db_version_iter {
            match db_version {
                Ok(dv) => db_versions.push(dv.version),
                Err(_e) => continue,
            }
        }
        db_versions
    }
}
