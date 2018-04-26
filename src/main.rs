#![feature(plugin)]
#![plugin(rocket_codegen)]

//standard includes
extern crate failure; //this crate has macros, but this current program doesn't make use of them
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

mod utils;

#[get("/")]
fn index() -> &'static str {
    trace!("index()");
    "Hello, world!"
}

#[derive(Debug, Serialize, Deserialize)]
enum ReservationStatus {
    Available,
    Reserved
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    device_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    device_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    device_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    reservation_status: Option<ReservationStatus>
}

#[get("/api/device/<name>")]
fn api_get_device(name: String) -> rocket_contrib::Json<Device> {
    trace!("api_get_device()");
    rocket_contrib::Json(Device {
        device_name:name,
        device_url: None,
        device_owner: None,
        comments: None,
        reservation_status: Some(ReservationStatus::Available),
    })
}

fn run(_config: &utils::types::Settings) -> Result<(), failure::Error> {
    trace!("Entry to top level run()");

    rocket::ignite().mount("/", routes![index, api_get_device]).launch();

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
