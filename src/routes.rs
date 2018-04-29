use database;
use failure;
use models;
use rocket;
use rocket_contrib;
use utils;

#[get("/")]
pub fn index() -> &'static str {
    trace!("index()");
    "Hello, world!"
}

#[get("/api/devices/<name>")]
pub fn api_get_device(name: String) -> rocket_contrib::Json<models::Device> {
    trace!("api_get_device()");
    rocket_contrib::Json(models::Device {
        device_name: name,
        reservation_status: Default::default(),
        ..Default::default()
    })
}

#[get("/api/devices")]
pub fn api_get_devices(
    config: rocket::State<utils::types::Settings>,
) -> Result<rocket_contrib::Json<Vec<models::Device>>, failure::Error> {
    trace!("api_get_devices()");
    let devices = database::get_devices(&*config)?;
    Ok(rocket_contrib::Json(devices))
}
