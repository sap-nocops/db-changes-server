#![feature(proc_macro_hygiene, decl_macro)]
extern crate dirs;
#[macro_use]
extern crate rocket;

use std::fs;

use argh::FromArgs;
//use serde::Serialize;
use rocket_contrib::json::Json;

use versions::Versions;

mod versions;

#[derive(FromArgs)]
/// Db changes configuration.
struct Arguments {
    /// apps folder
    #[argh(option)]
    folder: Option<String>,
}

static mut VERSION_API: Versions = Versions {
    apps_folder: String::new()
};

#[get("/<app_name>")]
fn versions(app_name: String) -> Json<Vec<String>> {
    unsafe {
        Json(
            VERSION_API.list(app_name)
        )
    }
}

fn main() {
    let args: Arguments = argh::from_env();
    unsafe {
        match args.folder {
            Some(x) => VERSION_API.apps_folder.push_str(&x),
            None => {
                match fs::canonicalize(dirs::home_dir().unwrap())
                {
                    Ok(hd) => VERSION_API.apps_folder.push_str(format!("{:?}/.db-changes/apps", hd).replace("\"", "").as_str()),
                    Err(_e) => panic!("Cannot set default apps path"),
                }
            }
        }
    }
    rocket::ignite().mount("/versions", routes![versions]).launch();
}
