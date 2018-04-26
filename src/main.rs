#![feature(plugin)]
#![plugin(rocket_codegen)]

//#[macro_use] //this crate has macros, currently unused
extern crate failure;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate clap;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use failure::ResultExt;

mod schema;
mod models;
mod utils;

pub fn establish_connection(
    config: &utils::types::Settings,
) -> Result<diesel::sqlite::SqliteConnection, failure::Error> {
    trace!("establish_connection()");
    let database_url = &config.database_url;
    Ok(diesel::sqlite::SqliteConnection::establish(database_url)
        .with_context(|_| format!("Error connecting to {}", database_url))?)
}

#[get("/")]
fn index() -> &'static str {
    trace!("index()");
    "Hello, world!"
}

#[get("/api/device/<name>")]
fn api_get_device(name: String) -> rocket_contrib::Json<models::Device> {
    trace!("api_get_device()");
    rocket_contrib::Json(models::Device {
        device_name: name,
        reservation_status: Some(Default::default()),
        ..Default::default()
    })
}

fn run(config: &utils::types::Settings) -> Result<(), failure::Error> {
    trace!("run()");

    let _ = establish_connection(config);

    rocket::ignite()
        .mount("/", routes![index, api_get_device])
        .launch();

    Ok(())
}

fn main() {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);

    if let Err(ref e) = run(&config) {
        use failure::Fail;
        let mut fail: &Fail = e.cause();
        error!("{}", fail);

        while let Some(cause) = fail.cause() {
            error!("caused by: {}", cause);
            fail = cause;
        }
        std::process::exit(1);
    }
}
