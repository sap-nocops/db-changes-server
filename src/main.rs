#![feature(proc_macro_hygiene, decl_macro)]
extern crate dirs;
#[macro_use]
extern crate rocket;

use argh::FromArgs;
mod api;
use api::Api;
use std::fs;

#[derive(FromArgs)]
/// Db changes configuration.
struct Arguments {
    /// server port. default: 8000
    #[argh(option)]
    port: Option<u16>,
    /// cache refresh time in seconds. default: 3600
    #[argh(option)]
    refresh_time: Option<u64>,
    /// path to db. default: ~/.db-changes/changes.db
    #[argh(option)]
    db_path: Option<String>,
    /// path to db version changes. default: ~/.db-changes/apps
    #[argh(option)]
    apps_path: Option<String>,
}

fn main() {
    let user_home: String;
    match fs::canonicalize(dirs::home_dir().unwrap())
    {
        Ok(hd) => user_home = hd.as_path().display().to_string().replace("\"", ""),
        Err(_e) => panic!("Cannot set default db path"),
    }
    let port: u16;
    let refresh_time: u64;
    let apps_path: String;
    let db_path: String;
    let args: Arguments = argh::from_env();
    match args.port {
        Some(x) => port = x,
        None => port = 8000,
    }
    match args.refresh_time {
        Some(x) => {
            if x < 1 {
                panic!("refresh_time cannot be less than 1 second");
            }
            refresh_time = x
        },
        None => refresh_time = 3600,
    }
    match args.db_path {
        Some(x) => db_path = x,
        None => db_path = format!("{}/.db-changes/changes.db", user_home),
    }
    match args.apps_path {
        Some(x) => apps_path = x,
        None => apps_path = format!("{}/.db-changes/apps", user_home),
    }
    let api: Api = Api{
        port,
        refresh_time,
        apps_path,
        db_path,
    };
    api.init();
}
