#![feature(proc_macro_hygiene, decl_macro)]
extern crate dirs;
#[macro_use]
extern crate rocket;

use argh::FromArgs;
mod api;
use api::Api;

#[derive(FromArgs)]
/// Db changes configuration.
struct Arguments {
    /// server port. default: 8000
    #[argh(option)]
    port: Option<u32>,
    /// cache refresh time in seconds. default: 3600
    #[argh(option)]
    refresh_time: Option<u32>,
}

fn main() {
    let port: u32;
    let refresh_time: u32;
    let args: Arguments = argh::from_env();
    match args.port {
        Some(x) => port = x,
        None => port = 8000,
    }
    match args.refresh_time {
        Some(x) => refresh_time = x,
        None => refresh_time = 3600,
    }
    let api: Api = Api{
        port,
        refresh_time,
    };
    api.init();
}
