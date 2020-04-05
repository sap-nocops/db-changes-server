#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

//use serde::Serialize;
use rocket_contrib::json::Json;
mod versions;
use versions::Versions;

use argh::FromArgs;

#[derive(FromArgs)]
/// Db changes configuration.
struct Arguments {
    /// apps folder
    #[argh(option)]
    folder: Option<String>,
}

static versionApi: Versions = Versions{
    appsFolder: "".to_string()
};

#[get("/<appName>")]
fn versions(appName: String) -> Json<Vec<String>> {
    Json(
        versionApi.list(appName)
    )
}

fn main() {
    let args: Arguments = argh::from_env();
    match args.folder {
        Some(x) => versionApi.appsFolder = x,
        None => versionApi.appsFolder = String::from("~/.db-changes/apps"),
    }
    rocket::ignite().mount("/versions", routes![versions]).launch();
}
