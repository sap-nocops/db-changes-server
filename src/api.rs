use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub mod versions;
pub mod changes;
pub mod cache;

use versions::VersionsApi;
use versions::new_sqlite_versions_api;
use changes::ChangesApi;
use changes::new_file_changes_api;
use cache::Cache;
use cache::new_hash_cache;

#[derive(Responder)]
pub enum StatusVersion {
    #[response(status = 200)]
    HttpOk(Json<Vec<String>>),
    #[response(status = 500)]
    HttpServerError(String),
    #[response(status = 404)]
    HttpNotFound(String),
}

#[derive(Responder)]
pub enum StatusChanges {
    #[response(status = 200)]
    HttpOk(String),
    #[response(status = 404)]
    HttpNotFound(String),
}

#[get("/<app_name>/<app_version>")]
fn list_versions(app_name: String, app_version: String, cache: State<Arc<Mutex<Box<dyn Cache + Send>>>>, versions_api: State<Box<dyn VersionsApi + Send + Sync>>) -> StatusVersion {
    let mut key = String::from("versions:");
    key.push_str(&app_name);
    key.push_str(":");
    key.push_str(&app_version);
    let mut m_cache = cache.lock().unwrap();
    match m_cache.get(&key) {
        Some(val) => {
            let split: Vec<&str> = val.split(",").collect();
            StatusVersion::HttpOk(Json(split.iter().map(|v| v.to_string()).collect()))
        }
        None => match versions_api.list(&app_name, &app_version) {
            Ok(versions) => {
                m_cache.insert(key, versions.join(","));
                if versions.len() == 0 {
                    return StatusVersion::HttpNotFound(format!("app {} {} not found", app_name, app_version));
                }
                StatusVersion::HttpOk(Json(versions))
            }
            Err(_e) => StatusVersion::HttpServerError(String::from("Error retrieving data")),
        }
    }
}

#[get("/<app_name>/<db_version>")]
fn changes(app_name: String, db_version: String, cache: State<Arc<Mutex<Box<dyn Cache + Send>>>>, changes_api: State<Box<dyn ChangesApi + Send + Sync>>) -> StatusChanges {
    let mut key = String::from("changes:");
    key.push_str(&app_name);
    key.push_str(":");
    key.push_str(&db_version);
    let mut m_cache = cache.lock().unwrap();
    match m_cache.get(&key) {
        Some(val) => StatusChanges::HttpOk(val),
        None => match changes_api.get(&app_name, &db_version) {
            Ok(val) => {
                m_cache.insert(key, val.clone());
                StatusChanges::HttpOk(val)
            }
            Err(_e) => StatusChanges::HttpNotFound(format!("db version {} of app {}", db_version, app_name)),
        }
    }
}

pub struct Api {
    pub port: u16,
    pub refresh_time: u64,
    pub apps_path: String,
    pub db_path: String,
}

impl Api {
    pub fn init(&self) {
        let cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(new_hash_cache())));
        self.clear_cache_periodically(&cache);
        let versions_api : Box<dyn VersionsApi + Send + Sync> = Box::new(new_sqlite_versions_api(&self.db_path));
        let changes: Box<dyn ChangesApi + Send + Sync> = Box::new(new_file_changes_api(&self.apps_path));
        rocket::ignite()
            .mount("/versions", routes![list_versions])
            .mount("/changes", routes![changes])
            .manage(versions_api)
            .manage(changes)
            .manage(cache)
            .launch();
    }

    fn clear_cache_periodically(&self, cache: &Arc<Mutex<Box<dyn Cache + Send>>>) {
        let cloned_cache = Arc::clone(&cache);
        let refresh_time = self.refresh_time;
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(refresh_time));
                cloned_cache.lock().unwrap().clear();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use versions::MockVersionsApi;
    use changes::MockChangesApi;
    use cache::MockCache;
    use mockall::predicate::eq;

    #[test]
    fn list_versions_insert_in_cache_when_not_found() {
        let app_name = String::from("name");
        let app_ver = String::from("1.0");
        let mut mock_ver = MockVersionsApi::default();
        mock_ver.expect_list()
            .with(eq("name"), eq("1.0"))
            .times(1)
            .returning(|_n, _v| Ok(vec!["v1".to_string(), "v2".to_string()]));
        let mut mock_cache = MockCache::default();
        mock_cache.expect_get()
            .with(eq("versions:name:1.0"))
            .times(1)
            .returning(|_v| None);
        mock_cache.expect_insert()
            .with(eq("versions:name:1.0".to_string()), eq("v1,v2".to_string()))
            .times(1)
            .returning(|_,_|());
        let arc_cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(mock_cache)));
        let box_ver:Box<dyn VersionsApi + Send + Sync> = Box::new(mock_ver);
        let rocket = rocket::ignite().manage(arc_cache).manage(box_ver);

        list_versions(app_name.clone(), app_ver.clone(), State::from(&rocket).unwrap(), State::from(&rocket).unwrap());
    }

    #[test]
    fn list_versions_get_from_cache() {
        let app_name = String::from("name");
        let app_ver = String::from("1.0");
        let mut mock_ver = MockVersionsApi::default();
        mock_ver.expect_list().times(0);
        let mut mock_cache = MockCache::default();
        mock_cache.expect_get()
            .with(eq("versions:name:1.0"))
            .times(1)
            .returning(|_v| Some("v1,v2".to_string()));
        mock_cache.expect_insert().times(0);
        let arc_cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(mock_cache)));
        let box_ver:Box<dyn VersionsApi + Send + Sync> = Box::new(mock_ver);
        let rocket = rocket::ignite().manage(arc_cache).manage(box_ver);

        list_versions(app_name.clone(), app_ver.clone(), State::from(&rocket).unwrap(), State::from(&rocket).unwrap());
    }

    #[test]
    fn get_changes_insert_in_cache_when_not_found() {
        let app_name = String::from("name");
        let db_ver = String::from("v1");
        let mut mock_ch = MockChangesApi::default();
        mock_ch.expect_get()
            .with(eq("name"), eq("v1"))
            .times(1)
            .returning(|_n, _v| Ok("CREATE TABLE antani (id int)".to_string()));
        let mut mock_cache = MockCache::default();
        mock_cache.expect_get()
            .with(eq("changes:name:v1"))
            .times(1)
            .returning(|_v| None);
        mock_cache.expect_insert()
            .with(eq("changes:name:v1".to_string()), eq("CREATE TABLE antani (id int)".to_string()))
            .times(1)
            .returning(|_,_|());
        let arc_cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(mock_cache)));
        let box_ver:Box<dyn ChangesApi + Send + Sync> = Box::new(mock_ch);
        let rocket = rocket::ignite().manage(arc_cache).manage(box_ver);

        changes(app_name.clone(), db_ver.clone(), State::from(&rocket).unwrap(), State::from(&rocket).unwrap());
    }

    #[test]
    fn get_changes_get_from_cache() {
        let app_name = String::from("name");
        let db_ver = String::from("v1");
        let mut mock_ch = MockChangesApi::default();
        mock_ch.expect_get().times(0);
        let mut mock_cache = MockCache::default();
        mock_cache.expect_get()
            .with(eq("changes:name:v1"))
            .times(1)
            .returning(|_v| Some("CREATE TABLE antani (id int)".to_string()));
        mock_cache.expect_insert().times(0);
        let arc_cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(mock_cache)));
        let box_ver:Box<dyn ChangesApi + Send + Sync> = Box::new(mock_ch);
        let rocket = rocket::ignite().manage(arc_cache).manage(box_ver);

        changes(app_name.clone(), db_ver.clone(), State::from(&rocket).unwrap(), State::from(&rocket).unwrap());
    }

    #[test]
    fn clear_cache() {
        let api = Api{
            port: 8000,
            refresh_time: 1,
            apps_path: "".to_string(),
            db_path: "".to_string()
        };
        let mut mock_cache = MockCache::default();
        mock_cache.expect_clear()
            .times(1)
            .returning(|| ());
        let arc_cache: Arc<Mutex<Box<dyn Cache + Send>>> = Arc::new(Mutex::new(Box::new(mock_cache)));

        api.clear_cache_periodically(&arc_cache);
        thread::sleep(Duration::from_millis(1030));
    }
}
