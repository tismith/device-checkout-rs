extern crate device_checkout_lib;
use device_checkout_lib::*;

extern crate tempfile;

#[test]
fn test_get_device() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/api/devices/unit1").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    assert!(response.body_string().unwrap().contains("http://unit1"));
}
