#![cfg_attr(feature = "cargo-clippy", allow(print_literal))]

use chrono;
use chrono::Offset;
use database;
use failure;
use models;
use pool;
use rocket;
use rocket_contrib;
use utils;

pub fn html_routes() -> Vec<rocket::Route> {
    routes![
        self::index,
        self::get_devices,
        self::post_devices,
        self::get_edit_devices,
        self::post_edit_devices,
        self::post_add_devices,
        self::post_delete_devices,
    ]
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![self::api_get_device, self::api_get_devices]
}

#[get("/")]
pub fn index() -> rocket::response::Redirect {
    trace!("index()");
    rocket::response::Redirect::to("/devices")
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/devices/<name>")]
pub fn api_get_device(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    name: String,
) -> Result<rocket_contrib::Json<models::Device>, rocket::response::status::Custom<String>> {
    trace!("api_get_device()");
    database::get_device(&*config, &*database, &name)
        .map_err(|_| {
            rocket::response::status::Custom(
                rocket::http::Status::InternalServerError,
                "500 Internal Server Error".to_string(),
            )
        })
        .and_then(|devices| {
            devices.ok_or_else(|| {
                rocket::response::status::Custom(
                    rocket::http::Status::NotFound,
                    "404 Not Found".to_string(),
                )
            })
        })
        .map(rocket_contrib::Json)
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/devices")]
pub fn api_get_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
) -> Result<rocket_contrib::Json<Vec<models::Device>>, failure::Error> {
    trace!("api_get_devices()");
    let devices = database::get_devices(&*config, &*database)?;
    Ok(rocket_contrib::Json(devices))
}

#[derive(Serialize)]
struct PerDeviceContext {
    device: models::Device,
    is_reserved: bool,
    updated_at_local: String,
}

#[derive(Serialize, Default)]
struct DevicesContext<'a> {
    devices: Vec<PerDeviceContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_message: Option<&'a str>,
}

fn format_device(device: models::Device) -> PerDeviceContext {
    let is_reserved = device.reservation_status == models::ReservationStatus::Reserved;
    let updated_at_local = chrono::DateTime::<chrono::Local>::from_utc(
        device.updated_at,
        chrono::Local::now().offset().fix(),
    );
    trace!("format_device");

    let updated_at_local = format!("{}", updated_at_local.format("%F %r"));
    PerDeviceContext {
        device,
        is_reserved,
        updated_at_local,
    }
}

fn gen_device_context<'a>(
    config: &'a utils::types::Settings,
    database: &'a database::DbConn,
    status_message: &'a Option<rocket::request::FlashMessage>,
) -> Result<DevicesContext<'a>, failure::Error> {
    trace!("gen_device_context");

    let mut success_message = None;
    let mut error_message = None;

    if let Some(ref status_message) = *status_message {
        if status_message.name() == "success" {
            success_message = Some(status_message.msg());
        } else {
            error_message = Some(status_message.msg());
        }
    }

    let devices: Vec<_> = database::get_devices(config, database)?
        .into_iter()
        .map(format_device)
        .collect();

    Ok(DevicesContext {
        devices,
        error_message,
        success_message,
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/devices")]
pub fn get_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    status_message: Option<rocket::request::FlashMessage>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_devices()");

    let context = gen_device_context(&*config, &*database, &status_message)?;
    Ok(rocket_contrib::Template::render("devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/editDevices")]
pub fn get_edit_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    status_message: Option<rocket::request::FlashMessage>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_edit_devices()");

    let context = gen_device_context(&*config, &*database, &status_message)?;
    Ok(rocket_contrib::Template::render("edit_devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/addDevices", data = "<device_add>")]
pub fn post_add_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_add: Result<rocket::request::LenientForm<models::DeviceInsert>, Option<String>>,
) -> rocket::response::Flash<rocket::response::Redirect> {
    trace!("post_add_devices()");

    let add_result = if let Ok(device_add) = device_add {
        let device = device_add.get();
        database::insert_device(&*config, &*database, device)
    } else {
        return rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to parse form data",
        );
    };

    match add_result {
        Ok(0) | Err(_) => rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to add device",
        ),
        _ => rocket::response::Flash::success(
            rocket::response::Redirect::to("/editDevices"),
            "Successfully added device",
        ),
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/deleteDevices", data = "<device_edit>")]
pub fn post_delete_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_edit: Result<rocket::request::LenientForm<models::DeviceDelete>, Option<String>>,
) -> rocket::response::Flash<rocket::response::Redirect> {
    trace!("post_delete_devices()");

    let update_result: Result<_, failure::Error> = if let Ok(device_edit) = device_edit {
        let device = device_edit.get();
        database::delete_device(&*config, &*database, device)
    } else {
        return rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to parse form data",
        );
    };

    return match update_result {
        Ok(0) | Err(_) => rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to delete device",
        ),
        _ => rocket::response::Flash::success(
            rocket::response::Redirect::to("/editDevices"),
            "Successfully deleted device",
        ),
    };
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/editDevices", data = "<device_edit>")]
pub fn post_edit_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_edit: Result<rocket::request::LenientForm<models::DeviceEdit>, Option<String>>,
) -> rocket::response::Flash<rocket::response::Redirect> {
    trace!("post_edit_devices()");

    let update_result: Result<_, failure::Error> = if let Ok(device_edit) = device_edit {
        let device = device_edit.get();
        database::edit_device(&*config, &*database, device)
    } else {
        return rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to parse form data",
        );
    };

    match update_result {
        Ok(0) | Err(_) => rocket::response::Flash::error(
            rocket::response::Redirect::to("/editDevices"),
            "Failed to update device",
        ),
        _ => rocket::response::Flash::success(
            rocket::response::Redirect::to("/editDevices"),
            "Successfully updated device",
        ),
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/devices", data = "<device_update>")]
pub fn post_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_update: Result<rocket::request::Form<models::DeviceUpdate>, Option<String>>,
) -> rocket::response::Flash<rocket::response::Redirect> {
    trace!("post_devices()");

    let update_result = if let Ok(device_update) = device_update {
        let mut device = device_update.into_inner();
        //save the old reservation status around for the sql query
        let current_reservation_status = device.reservation_status;

        //toggle the reservation status
        device.reservation_status = !device.reservation_status;

        //blank out the owner and comments if we're returning it
        if device.reservation_status == models::ReservationStatus::Available {
            device.device_owner = None;
            device.comments = None;
        }

        database::update_device(&*config, &*database, &device, current_reservation_status)
    } else {
        return rocket::response::Flash::error(
            rocket::response::Redirect::to("/devices"),
            "Failed to parse form data",
        );
    };

    match update_result {
        Ok(0) | Err(_) => rocket::response::Flash::error(
            rocket::response::Redirect::to("/devices"),
            "Failed to update device",
        ),
        _ => rocket::response::Flash::success(
            rocket::response::Redirect::to("/devices"),
            "Successfully updated device",
        ),
    }
}

pub fn rocket(config: utils::types::Settings) -> Result<rocket::Rocket, failure::Error> {
    let mut rocket_builder =
        rocket::config::Config::build(rocket::config::Environment::Production).port(config.port);

    if let Some(ref template_dir) = config.template_dir {
        rocket_builder = rocket_builder.extra("template_dir", template_dir.as_ref());
    }

    let rocket_config = rocket_builder.finalize()?;

    Ok(rocket::custom(rocket_config, true)
        .manage(pool::init_pool(&config))
        .manage(config)
        .attach(rocket_contrib::Template::fairing())
        .mount("/", html_routes())
        .mount("/api/", api_routes()))
}
