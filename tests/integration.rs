extern crate device_checkout_lib;
use device_checkout_lib::*;

extern crate tempfile;

#[test]
fn test_api_get_device() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/api/devices/unit1").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    assert!(response.body_string().unwrap().contains("http://unit1"));

    let response = client.get("/api/devices/some_unknown_device").dispatch();
    assert_eq!(response.status(), rocket::http::Status::NotFound);
}

#[test]
fn test_api_get_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/api/devices").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();
    assert!(body.contains("http://unit1"));
    assert!(body.contains("http://unit2"));
}

#[test]
fn test_html_get_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/devices").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();
    assert!(body.contains("http://unit1"));
    assert!(body.contains("http://unit2"));
}

#[test]
fn test_html_get_edit_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/editDevices").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();
    assert!(body.contains("http://unit1"));
    assert!(body.contains("http://unit2"));
}
