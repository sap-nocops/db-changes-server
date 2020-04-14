use rocket_contrib::json::Json;
use rocket::State;

pub mod versions;
pub mod changes;

use versions::Versions;
use changes::Changes;

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
fn list_versions(app_name: String, app_version: String, versions_api: State<'_, Versions>) -> StatusVersion {
    match versions_api.list(&app_name, &app_version) {
        Ok(versions) => {
            if versions.len() == 0 {
                return StatusVersion::HttpNotFound(format!("app {} {} not found", app_name, app_version))
            }
            StatusVersion::HttpOk(Json(versions))
        }
        Err(_e) => StatusVersion::HttpServerError(String::from("Error retrieving data")),
    }
}

#[get("/<app_name>/<db_version>")]
fn changes(app_name: String, db_version: String, changes_api: State<'_, Changes>) -> StatusChanges {
    match changes_api.get(&app_name, &db_version) {
        Ok(val) => StatusChanges::HttpOk(val),
        Err(_e) => StatusChanges::HttpNotFound(format!("db version {} of app {}", db_version, app_name)),
    }
}

pub struct Api {
    pub port: u32,
    pub refresh_time: u32,
    pub apps_path: String,
    pub db_path: String,
}

impl Api {
    pub fn init(&self) {
        let versions_api = Versions {
            db_path: self.db_path.clone()
        };
        let changes = Changes {
            apps_path: self.apps_path.clone()
        };
        rocket::ignite()
            .mount("/versions", routes![list_versions])
            .mount("/changes", routes![changes])
            .manage(versions_api)
            .manage(changes)
            .launch();
    }
}
