use diesel;
use failure;
use models;
use utils;

use diesel::prelude::*;
use failure::ResultExt;

pub fn establish_connection(
    config: &utils::types::Settings,
) -> Result<diesel::sqlite::SqliteConnection, failure::Error> {
    trace!("establish_connection()");
    let database_url = &config.database_url;
    Ok(diesel::sqlite::SqliteConnection::establish(database_url)
        .with_context(|_| format!("Error connecting to {}", database_url))?)
}

pub fn get_devices(config: &utils::types::Settings) -> Result<Vec<models::Device>, failure::Error> {
    use self::diesel::prelude::*;
    use schema::devices::dsl::*;

    let connection = establish_connection(config)?;
    let results = devices
        .load::<models::Device>(&connection)
        .with_context(|_| format!("Error loading devices"))?;

    Ok(results)
}
