use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub mod versions;
pub mod changes;
pub mod cache;

use versions::Versions;
use changes::Changes;
use cache::Cache;

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
fn list_versions(app_name: String, app_version: String, cache: State<Arc<Mutex<Cache>>>, versions_api: State<Versions>) -> StatusVersion {
    let key = vec![String::from("versions"), app_name.clone(), app_version.clone()];
    let mut m_cache = cache.lock().unwrap();
    match m_cache.get_vec(&key) {
        Some(val) => StatusVersion::HttpOk(Json(val.clone())),
        None => match versions_api.list(&app_name, &app_version) {
            Ok(versions) => {
                m_cache.insert_vec(key, versions.clone());
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
fn changes(app_name: String, db_version: String, cache: State<'_, Arc<Mutex<Cache>>>, changes_api: State<'_, Changes>) -> StatusChanges {
    let key = vec![String::from("changes"), app_name.clone(), db_version.clone()];
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
        let cache = Arc::new(Mutex::new(Cache::new()));
        let cloned_cache = Arc::clone(&cache);
        let refresh_time = self.refresh_time;
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(refresh_time));
                cloned_cache.lock().unwrap().clear();
            }
        });
        let versions_api = Versions::new(&self.db_path);
        let changes = Changes::new(&self.apps_path);
        rocket::ignite()
            .mount("/versions", routes![list_versions])
            .mount("/changes", routes![changes])
            .manage(versions_api)
            .manage(changes)
            .manage(cache)
            .launch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use versions::MockVersions;
    use cache::MockCache;
    use mockall::predicate::eq;

    struct Testolo {
        a:i32
    }

    impl Testolo{
        fn ciao(&self) -> String {
            format!("ciao: {}", self.a)
        }
    }

    #[get("/")]
    fn vvv(c: State<Testolo>) -> super::StatusChanges {
        super::StatusChanges::HttpOk(c.ciao())
    }

    #[test]
    fn list_versions_insert_in_cache_when_not_found() {
        let app_name = String::from("name");
        let app_ver = String::from("1.0");
        let key = vec![String::from("versions"), app_name.clone(), app_ver.clone()];
        let mut mock_ver = MockVersions::default();
        mock_ver.expect_list()
            .with(eq("name"), eq("1.0"))
            .times(1)
            .returning(|_n, _v| Ok(vec!["v1".to_string(), "v2".to_string()]));
        let mut mock_cache = MockCache::default();
        mock_cache.expect_get_vec()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_v| None);
        /*mock_cache.expect_insert_vec()
            .with(eq(key), eq(value.clone()))
            .times(1);*/
        let rocket = rocket::ignite().manage(Arc::new(Mutex::new(mock_cache)));

        list_versions(app_name.clone(), app_ver.clone(), State::from(&rocket).unwrap(), State::from(&rocket).unwrap());
    }

    #[test]
    fn ciao_test() {
        let testolo = Testolo{
            a: 0,
        };
        let rocket = rocket::ignite().mount("/", routes![vvv]).manage(testolo);

        vvv(State::from(&rocket).unwrap());
    }
}
