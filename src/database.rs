use diesel;
use failure;
use models;
use utils;

use diesel::prelude::*;
use failure::ResultExt;

///Establish a database connection
pub fn establish_connection(
    config: &utils::types::Settings,
) -> Result<diesel::sqlite::SqliteConnection, failure::Error> {
    trace!("establish_connection()");
    let database_url = &config.database_url;
    Ok(diesel::sqlite::SqliteConnection::establish(database_url)
        .with_context(|_| format!("Error connecting to {}", database_url))?)
}

///Get all the devices
pub fn get_devices(config: &utils::types::Settings) -> Result<Vec<models::Device>, failure::Error> {
    use self::diesel::prelude::*;
    use schema::devices::dsl::*;

    let connection = establish_connection(config)?;
    let results = devices
        .load::<models::Device>(&connection)
        .with_context(|_| format!("Error loading devices"))?;

    Ok(results)
}

///Lookup a single device
pub fn get_device(
    config: &utils::types::Settings,
    requested_name: &str,
) -> Result<Option<models::Device>, failure::Error> {
    use self::diesel::prelude::*;
    use schema::devices::dsl::*;

    let connection = establish_connection(config)?;
    let result = devices
        .filter(device_name.eq(requested_name))
        .load::<models::Device>(&connection)
        .with_context(|_| format!("Error loading devices"))?
        .into_iter()
        .next();

    Ok(result)
}

///Updates a device, designed for the common case on the main http form
pub fn update_device(
    config: &utils::types::Settings,
    device_update: &models::DeviceUpdate,
) -> Result<usize, failure::Error> {
    use self::diesel::prelude::*;
    use schema::devices::dsl::*;

    let connection = establish_connection(config)?;

    Ok(diesel::update(devices.filter(id.eq(&device_update.id)))
        .set((
            device_owner.eq(&device_update.device_owner),
            comments.eq(&device_update.comments),
            reservation_status.eq(&device_update.reservation_status),
        ))
        .execute(&connection)?)
}

///Edits the details specific to the device, i.e the name and url
pub fn edit_device(
    config: &utils::types::Settings,
    device_edit: &models::DeviceEdit,
) -> Result<usize, failure::Error> {
    use self::diesel::prelude::*;
    use schema::devices::dsl::*;

    let connection = establish_connection(config)?;

    Ok(diesel::update(devices.filter(id.eq(&device_edit.id)))
        .set((
            device_name.eq(&device_edit.device_name),
            device_url.eq(&device_edit.device_url),
        ))
        .execute(&connection)?)
}
