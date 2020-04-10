use rocket_contrib::json::Json;
use rocket::State;

pub mod versions;
pub mod changes;

use versions::Versions;
use versions::Changes;

#[get("/<app_name>/<app_version>")]
fn list_versions(app_name: String, app_version: String, versions_api: State<'_, Versions>) -> Json<Vec<String>> {
    Json(
        versions_api.list(app_name, app_version)
    )
}

#[get("/<app_name>/<app_version>/<db_version>")]
fn changes(app_name: String, app_version: String, db_version: String, changes_api: State<'_, Changes>) -> Json<Vec<String>> {
    Json(
        changes_api.get(app_name, app_version, db_version)
    )
}

pub struct Api {
    pub port: u32,
    pub refresh_time: u32,
    pub apps_path: String,
}

impl Api {
    pub fn init(&self) {
        let versions_api = Versions {
            apps_path: self.apps_path.clone()
        };
        rocket::ignite()
            .mount("/versions", routes![list_versions])
            .mount("/changes", routes![changes])
            .manage(versions_api)
            .launch();
        //rocket::ignite().mount("/changes", routes![self.changes]).launch();
    }
}
