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
    port: Option<u32>,
    /// cache refresh time in seconds. default: 3600
    #[argh(option)]
    refresh_time: Option<u32>,
    /// path to apps db changes. default: ~/.db-changes/apps
    #[argh(option)]
    apps_path: Option<String>,
}

fn main() {
    let port: u32;
    let refresh_time: u32;
    let apps_path: String;
    let args: Arguments = argh::from_env();
    match args.port {
        Some(x) => port = x,
        None => port = 8000,
    }
    match args.refresh_time {
        Some(x) => refresh_time = x,
        None => refresh_time = 3600,
    }
    match args.apps_path {
        Some(x) => apps_path = x,
        None => {
            match fs::canonicalize(dirs::home_dir().unwrap())
            {
                Ok(hd) => apps_path = format!("{:?}/.db-changes/apps", hd).replace("\"", ""),
                Err(_e) => panic!("Cannot set default apps path"),
            }
        },
    }
    let api: Api = Api{
        port,
        refresh_time,
        apps_path
    };
    api.init();
}
