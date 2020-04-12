use std::fs;
use rusqlite::{params, Connection};

pub struct Versions {
    pub db_path: String,
}

impl Versions {
    pub fn list(&self, app_name: String, app_version: String) -> Vec<String> {
        let conn = Connection::open(self.db_path.clone())?;
        let mut stmt = conn.prepare("SELECT dv.version FROM apps a JOIN apps_db_versions adv\
         ON a.id = adv.app_id JOIN db_versions dv ON dv.id = adv.db_id\
         WHERE av.name = ? AND av.version = ?");
        let db_version_iter = stmt.query_map(params![app_name, app_version], |row| {
            Ok(row.get(0),)
        });
        let mut db_versions = Vec::new();
        for db_version in db_version_iter {
            db_versions.push(db_version.unwrap());
        }
        db_versions
    }
}
