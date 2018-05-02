use database;
use failure;
use models;
use rocket;
use rocket_contrib;
use utils;

#[get("/")]
pub fn index() -> rocket::response::Redirect {
    trace!("index()");
    rocket::response::Redirect::to("/devices")
}

#[get("/devices/<name>")]
pub fn api_get_device(
    config: rocket::State<utils::types::Settings>,
    name: String,
) -> Result<rocket_contrib::Json<models::Device>, rocket::response::status::Custom<String>> {
    trace!("api_get_device()");
    database::get_device(&*config, &name)
        .map_err(|_| {
            rocket::response::status::Custom(
                rocket::http::Status::InternalServerError,
                "500 Internal Server Error".to_string(),
            )
        })
        .and_then(|devices| {
            devices.ok_or(rocket::response::status::Custom(
                rocket::http::Status::NotFound,
                "404 Not Found".to_string(),
            ))
        })
        .map(rocket_contrib::Json)
}

#[get("/devices")]
pub fn api_get_devices(
    config: rocket::State<utils::types::Settings>,
) -> Result<rocket_contrib::Json<Vec<models::Device>>, failure::Error> {
    trace!("api_get_devices()");
    let devices = database::get_devices(&*config)?;
    Ok(rocket_contrib::Json(devices))
}

#[derive(Serialize)]
struct PerDeviceContext {
    device: models::Device,
    button_string: String,
    button_class: String,
}

#[derive(Serialize, Default)]
struct DevicesContext {
    devices: Vec<PerDeviceContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_message: Option<String>,
}

fn format_device(device: models::Device) -> PerDeviceContext {
    let button_string = match device.reservation_status {
        models::ReservationStatus::Reserved => "RETURN".to_string(),
        _ => "CLAIM".to_string(),
    };
    let button_class = match device.reservation_status {
        models::ReservationStatus::Reserved => "btn-danger".to_string(),
        _ => "btn-primary".to_string(),
    };
    PerDeviceContext {
        device: device,
        button_string,
        button_class,
    }
}

#[get("/devices")]
pub fn get_devices(
    config: rocket::State<utils::types::Settings>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_devices()");

    let devices: Vec<_> = database::get_devices(&*config)?
        .into_iter()
        .map(format_device)
        .collect();
    let context = DevicesContext {
        devices,
        ..Default::default()
    };
    Ok(rocket_contrib::Template::render("devices", &context))
}

#[post("/devices", data = "<device_update>")]
pub fn post_devices(
    config: rocket::State<utils::types::Settings>,
    device_update: rocket::request::Form<models::DeviceUpdate>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("post_devices()");

    let mut device = device_update.into_inner();
    //toggle the reservation status
    if device.reservation_status == models::ReservationStatus::Available {
        device.reservation_status = models::ReservationStatus::Reserved;
    } else {
        device.reservation_status = models::ReservationStatus::Available;
    }

    //blank out the owner if we're returning it
    if device.reservation_status == models::ReservationStatus::Available {
        device.device_owner = None;
    }

    let update_result = database::update_device(&*config, &device);

    let mut success_message = None;
    let mut error_message = None;
    if let Ok(_) = update_result {
        success_message = Some("Device updated successufully".to_string());
    } else if let Err(e) = update_result {
        error_message = Some(format!("{}", e));
    }

    let devices: Vec<_> = database::get_devices(&*config)?
        .into_iter()
        .map(format_device)
        .collect();
    let context = DevicesContext {
        devices,
        error_message,
        success_message,
    };

    Ok(rocket_contrib::Template::render("devices", &context))
}
