#![cfg_attr(feature = "cargo-clippy", allow(print_literal))]

use chrono;
use chrono::Offset;
use database;
use failure;
use failure::ResultExt;
use models;
use pool;
use rocket;
use rocket_contrib;
use std;
use utils;

pub fn html_routes() -> Vec<rocket::Route> {
    routes![
        self::index,
        self::get_devices,
        self::post_devices,
        self::get_edit_devices,
        self::post_edit_devices,
        self::post_add_devices
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
    error_message: Option<std::borrow::Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_message: Option<std::borrow::Cow<'a, str>>,
}

fn format_device<'a>(device: models::Device) -> PerDeviceContext {
    let is_reserved = device.reservation_status == models::ReservationStatus::Reserved;
    let updated_at_local = chrono::DateTime::<chrono::Local>::from_utc(
        device.updated_at,
        chrono::Local::now().offset().fix(),
    );
    let updated_at_local = format!("{}", updated_at_local.format("%F %r"));
    PerDeviceContext {
        device,
        is_reserved,
        updated_at_local,
    }
}

fn gen_device_context<'a, T>(
    config: &utils::types::Settings,
    database: &database::DbConn,
    db_result: &Option<Result<T, failure::Error>>,
) -> Result<DevicesContext<'a>, failure::Error> {
    trace!("gen_device_context");

    let mut success_message = None;
    let mut error_message = None;

    if let Some(db_result) = db_result {
        if db_result.is_ok() {
            success_message = Some("Device updated successufully".into());
        } else if let Err(e) = db_result {
            error_message = Some(format!("{}", e).into());
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
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_devices()");

    let context = gen_device_context::<usize>(&*config, &*database, &None)?;
    Ok(rocket_contrib::Template::render("devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/editDevices")]
pub fn get_edit_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("get_edit_devices()");

    let context = gen_device_context::<usize>(&*config, &*database, &None)?;
    Ok(rocket_contrib::Template::render("edit_devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/addDevices", data = "<device_add>")]
pub fn post_add_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_add: Result<rocket::request::LenientForm<models::DeviceInsert>, Option<String>>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("post_add_devices()");

    let mut add_result = if let Ok(device_add) = device_add {
        let device = device_add.get();
        database::insert_device(&*config, &*database, device)
            .context("Failed to add device")
            .map_err(|e| e.into())
    } else {
        Err(failure::err_msg("Failed to parse form data"))
    };

    if let Ok(0) = add_result {
        add_result = Err(failure::err_msg("Failed to update device"));
    }

    let context = gen_device_context(&*config, &*database, &Some(add_result))?;
    Ok(rocket_contrib::Template::render("edit_devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/editDevices", data = "<device_edit>")]
pub fn post_edit_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_edit: Result<rocket::request::Form<models::DeviceEdit>, Option<String>>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("post_edit_devices()");

    let mut update_result = if let Ok(device_edit) = device_edit {
        let device = device_edit.get();
        if device.save.is_some() {
            trace!("saving");
            database::edit_device(&*config, &*database, &device)
                .context("Failed to save device")
                .map_err(|e| e.into())
        } else if device.delete.is_some() {
            trace!("deleting");
            database::delete_device(&*config, &*database, &device)
                .context("Failed to delete device")
                .map_err(|e| e.into())
        } else {
            Err(failure::err_msg("Unknown form action"))
        }
    } else {
        Err(failure::err_msg("Failed to parse form data"))
    };

    if let Ok(0) = update_result {
        update_result = Err(failure::err_msg("Failed to update device"));
    }

    let context = gen_device_context(&*config, &*database, &Some(update_result))?;
    Ok(rocket_contrib::Template::render("edit_devices", &context))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/devices", data = "<device_update>")]
pub fn post_devices(
    config: rocket::State<utils::types::Settings>,
    database: pool::DbConn,
    device_update: Result<rocket::request::Form<models::DeviceUpdate>, Option<String>>,
) -> Result<rocket_contrib::Template, failure::Error> {
    trace!("post_devices()");

    let mut update_result = if let Ok(device_update) = device_update {
        let mut device = device_update.into_inner();
        //toggle the reservation status
        let current_reservation_status = device.reservation_status;
        if device.reservation_status == models::ReservationStatus::Available {
            device.reservation_status = models::ReservationStatus::Reserved;
        } else {
            device.reservation_status = models::ReservationStatus::Available;
        }

        //blank out the owner and comments if we're returning it
        if device.reservation_status == models::ReservationStatus::Available {
            device.device_owner = None;
            device.comments = None;
        }

        database::update_device(&*config, &*database, &device, &current_reservation_status)
            .context("Failed to save device")
            .map_err(|e| e.into())
    } else {
        Err(failure::err_msg("Failed to parse form data"))
    };

    if let Ok(0) = update_result {
        update_result = Err(failure::err_msg("Failed to update device"));
    }

    let context = gen_device_context(&*config, &*database, &Some(update_result))?;
    Ok(rocket_contrib::Template::render("devices", &context))
}

pub fn rocket(config: utils::types::Settings) -> Result<rocket::Rocket, failure::Error> {
    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production)
        .port(config.port)
        .finalize()?;

    Ok(rocket::custom(rocket_config, true)
        .manage(pool::init_pool(&config))
        .manage(config)
        .attach(rocket_contrib::Template::fairing())
        .mount("/", html_routes())
        .mount("/api/", api_routes()))
}
