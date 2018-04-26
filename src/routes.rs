use rocket_contrib;
use models;

#[get("/")]
pub fn index() -> &'static str {
    trace!("index()");
    "Hello, world!"
}

#[get("/api/device/<name>")]
pub fn api_get_device(name: String) -> rocket_contrib::Json<models::Device> {
    trace!("api_get_device()");
    rocket_contrib::Json(models::Device {
        device_name: name,
        reservation_status: Some(Default::default()),
        ..Default::default()
    })
}

