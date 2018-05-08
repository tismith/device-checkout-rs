use diesel;
use failure;
use models;
use std;
use utils;

use self::diesel::prelude::*;
use failure::ResultExt;
use schema::devices;
use schema::devices::dsl::*;

pub type DbConn = diesel::sqlite::SqliteConnection;

embed_migrations!();

pub fn run_migrations(config: &utils::types::Settings) -> Result<(), failure::Error> {
    let connection = establish_connection(config)?;
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())?;
    Ok(())
}

fn establish_connection(config: &utils::types::Settings) -> Result<DbConn, failure::Error> {
    trace!("establish_connection()");
    Ok(DbConn::establish(&config.database_url)
        .with_context(|_| format!("Error connection to {}", &config.database_url))?)
}

///Get all the devices
pub fn get_devices(
    _config: &utils::types::Settings,
    database: &DbConn,
) -> Result<Vec<models::Device>, failure::Error> {
    Ok(devices
        .load::<models::Device>(database)
        .with_context(|_| "Error loading devices".to_string())?)
}

///Lookup a single device
pub fn get_device(
    _config: &utils::types::Settings,
    database: &DbConn,
    requested_name: &str,
) -> Result<Option<models::Device>, failure::Error> {
    Ok(devices
        .filter(device_name.eq(requested_name))
        .load::<models::Device>(database)
        .with_context(|_| "Error loading devices".to_string())?
        .into_iter()
        .next())
}

///Updates a device, designed for the common case on the main http form
pub fn update_device(
    _config: &utils::types::Settings,
    database: &DbConn,
    device_update: &models::DeviceUpdate,
) -> Result<usize, failure::Error> {
    Ok(diesel::update(devices.filter(id.eq(&device_update.id)))
        .set((
            device_owner.eq(&device_update.device_owner),
            comments.eq(&device_update.comments),
            reservation_status.eq(&device_update.reservation_status),
        ))
        .execute(database)?)
}

///Edits the details specific to the device, i.e the name and url
pub fn edit_device(
    _config: &utils::types::Settings,
    database: &DbConn,
    device_edit: &models::DeviceEdit,
) -> Result<usize, failure::Error> {
    Ok(diesel::update(devices.filter(id.eq(&device_edit.id)))
        .set((
            device_name.eq(&device_edit.device_name),
            device_url.eq(&device_edit.device_url),
        ))
        .execute(database)?)
}

///Edits the details specific to the device, i.e the name and url
pub fn delete_device(
    _config: &utils::types::Settings,
    database: &DbConn,
    device_edit: &models::DeviceEdit,
) -> Result<usize, failure::Error> {
    Ok(diesel::delete(devices.filter(id.eq(&device_edit.id))).execute(database)?)
}

///Inserts a new device
pub fn insert_device(
    _config: &utils::types::Settings,
    database: &DbConn,
    device_insert: &models::DeviceInsert,
) -> Result<usize, failure::Error> {
    Ok(diesel::insert_into(devices::table)
        .values(device_insert)
        .execute(database)?)
}
