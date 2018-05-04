#![feature(plugin, custom_derive, custom_attribute)]
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
#[macro_use]
extern crate diesel_derive_enum;
extern crate chrono;

mod database;
mod database_pool;
mod models;
mod routes;
mod schema;
mod utils;

fn run(config: utils::types::Settings) -> Result<(), failure::Error> {
    trace!("run()");

    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production)
        .port(config.port)
        .finalize()?;

    rocket::custom(rocket_config, true)
        .manage(database_pool::init_pool(&config))
        .manage(config)
        .attach(rocket_contrib::Template::fairing())
        .mount(
            "/",
            routes![
                routes::index,
                routes::get_devices,
                routes::post_devices,
                routes::get_edit_devices,
                routes::post_edit_devices,
                routes::post_add_devices,
            ],
        )
        .mount(
            "/api/",
            routes![routes::api_get_device, routes::api_get_devices,],
        )
        .launch();

    Ok(())
}

fn main() {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);

    if let Err(ref e) = run(config) {
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
