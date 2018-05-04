use diesel;
use rocket;
use utils;

use rocket::request::FromRequest;
use std::ops::Deref;

// An alias to the type for a pool of Diesel SQLite connections.
pub type SqlitePool =
    diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>;

/// Initializes a database pool.
pub fn init_pool(config: &utils::types::Settings) -> SqlitePool {
    let database_url = config.database_url.clone();
    let manager =
        diesel::r2d2::ConnectionManager::<diesel::sqlite::SqliteConnection>::new(database_url);
    diesel::r2d2::Pool::new(manager).expect("Database pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(
    pub  diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>,
    >,
);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = request.guard::<rocket::State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => rocket::Outcome::Success(DbConn(conn)),
            Err(_) => rocket::Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = diesel::sqlite::SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
