use database;
use failure;
use models;
use rocket;
use rocket_contrib;
use std;
use utils;

#[get("/")]
pub fn index() -> rocket::response::Redirect {
    trace!("index()");
    rocket::response::Redirect::to("/devices")
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

#[get("/devices")]
pub fn get_devices(
    config: rocket::State<utils::types::Settings>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_devices()");

    #[derive(Serialize)]
    struct GetDevicesContext {
        device: models::Device,
        button_string: String,
        button_class: String,
    }

    let devices: Vec<_> = database::get_devices(&*config)?
        .into_iter()
        .map(|d| {
            let button_string = match d.reservation_status {
                models::ReservationStatus::Reserved => "RETURN".to_string(),
                _ => "CLAIM".to_string(),
            };
            let button_class = match d.reservation_status {
                models::ReservationStatus::Reserved => "btn-danger".to_string(),
                _ => "btn-primary".to_string(),
            };
            GetDevicesContext {
                device: d,
                button_string,
                button_class,
            }
        })
        .collect();
    let mut context = std::collections::HashMap::new();
    context.insert("devices", devices);
    Ok(rocket_contrib::Template::render("devices", &context))
}
