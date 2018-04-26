use diesel;
use failure;
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
