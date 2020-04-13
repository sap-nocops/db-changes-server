use rusqlite::{params, Connection, Error};

#[derive(Debug)]
struct DbVersion {
    version: String,
}

pub struct Versions {
    pub db_path: String,
}

impl Versions {
    pub fn list(&self, app_name: String, app_version: String) -> Result<Vec<String>, Error> {
        let conn = Connection::open(self.db_path.clone())?;
        let mut stmt = conn.prepare("SELECT dv.version FROM apps a JOIN apps_db_versions adv
         ON a.id = adv.app_id JOIN db_versions dv ON dv.id = adv.db_id
         WHERE a.name = ? AND a.version = ?")?;
        let db_version_iter = stmt.query_map(params![app_name, app_version], |row| {
            Ok(DbVersion{version: row.get(0)?,})
        })?;
        let mut db_versions = Vec::new();
        for db_version in db_version_iter {
            match db_version {
                Ok(dv) => db_versions.push(dv.version),
                Err(_e) => continue,
            }
        }
        Ok(db_versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_db_versions_for_app() {
        let version_api = Versions{
            db_path: String::from("test_data/test.db")
        };

        let db_versions = version_api.list(String::from("caponzoniere"), String::from("1.0.0"));

        let mut expected = Vec::new();
        expected.push(String::from("v1"));
        assert_eq!(db_versions, Ok(expected));
    }

    #[test]
    fn empty_when_non_existing_app() {
        let version_api = Versions{
            db_path: String::from("test_data/test.db")
        };

        let db_versions = version_api.list(String::from("non-existing-app"), String::from("1.0.0"));

        assert_eq!(db_versions, Ok(Vec::new()));
    }

    #[test]
    fn empty_when_non_existing_app_version() {
        let version_api = Versions{
            db_path: String::from("test_data/test.db")
        };

        let db_versions = version_api.list(String::from("caponzoniere"), String::from("non_existing"));

        assert_eq!(db_versions, Ok(Vec::new()));
    }
}
