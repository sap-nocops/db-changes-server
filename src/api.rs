use rocket_contrib::json::Json;
use rocket::State;

pub mod versions;
use versions::Versions;

#[get("/<app_name>/<app_version>")]
fn list_versions(app_name: String, app_version: String, versions_api: State<'_, Versions>) -> Json<Vec<String>> {
    Json(
        versions_api.list(app_name, app_version)
    )
}

pub struct Api {
    pub port: u32,
    pub refresh_time: u32,
    pub apps_path: String,
}

impl Api {
    /*
    #[get("/<app_name>/<version>")]
    fn changes(&self,app_name: String, version: String) -> Json<Vec<String>> {
        unsafe {
            Json(
                self.versions.list(app_name)
            )
        }
    }
    */

    pub fn init(&self) {
        let versions_api = Versions {
            apps_path: self.apps_path.clone()
        };
        rocket::ignite()
            .mount("/versions", routes![list_versions])
            .manage(versions_api)
            .launch();
        //rocket::ignite().mount("/changes", routes![self.changes]).launch();
    }
}
