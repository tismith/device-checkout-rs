extern crate device_checkout_lib;
use device_checkout_lib::*;

extern crate tempfile;
extern crate victoria_dom;

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
    let body = response.body_string().unwrap();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["device_url"], "http://unit1");

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
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v[0]["device_url"], "http://unit1");
    assert_eq!(v[1]["device_url"], "http://unit2");
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
    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"form[name="unit1"] a[href="http://unit1"]"#)
        .expect("failed to find unit1");
    let _ = dom.at(r#"form[name="unit2"] a[href="http://unit2"]"#)
        .expect("failed to find unit2");
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
    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"form[name="unit1"] input[name="device_url"][value="http://unit1"]"#)
        .expect("failed to find unit1");
    let _ = dom.at(r#"form[name="unit2"] input[name="device_url"][value="http://unit2"]"#)
        .expect("failed to find unit2");
}

#[test]
fn test_html_post_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);

    let body = response.body_string().unwrap();
    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form[name="unit1"] input[name="device_owner"][value="Owner"]"#)
        .expect("failed to find owner");

    let _ = dom.at(r#"form[name="unit1"] input[name="reservation_status"][value="Reserved"]"#)
        .expect("failed to find reservation status");

    let _ = dom.at(r#"form[name="unit1"] input[name="comments"][value="xyzzy"]"#)
        .expect("failed to find comments");
}

#[test]
fn test_html_edit_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/editDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_name=testunit&device_url=testurl&save=SAVE"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form[name="testunit"] input[name="device_url"][value="testurl"]"#)
        .expect("failed to find edited device");
}

#[test]
fn test_html_edit_devices_delete() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/editDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_name=testunit&device_url=testurl&delete=DELETE"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    assert!(dom.at(r#"form[name="unit1"]"#).is_none());
}

#[test]
fn test_html_add_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/addDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"device_name=testunit&device_url=testurl&add=ADD"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form[name="testunit"] input[name="device_url"][value="testurl"]"#)
        .expect("failed to find added device");
}

#[test]
fn test_get_root() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client.get("/").dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);
}

#[test]
fn test_reserve_already_reserved() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let mut location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    //reserve unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"form[name="unit1"] input[name="reservation_status"][value="Reserved"]"#)
        .expect("failed to find reservation status");

    //reserve unit2
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner2&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }
    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#error_message"#)
        .expect("failed to find error message");
    assert!(dom.at(r#"#success_message"#).is_none());

    let _ = dom.at(r#"form[name="unit1"] input[name="device_owner"][value="Owner"]"#)
        .expect("failed to find owner");
    assert!(
        dom.at(r#"form[name="unit1"] input[name="device_owner"][value="Owner2"]"#)
            .is_none()
    );

    assert_eq!(response.status(), rocket::http::Status::Ok);
}

#[test]
fn test_returning_clears_fields() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let mut cookies;
    let mut location;
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    //reserve unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config.clone()).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);

    //return unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&reservation_status=Reserved"#)
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::SeeOther);

    cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    location = response
        .headers()
        .get("Location")
        .next()
        .unwrap()
        .to_string();

    //manually follow the redirection with a new client
    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    let mut response = request.dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    //test that the old values for the reservation are gone
    assert!(
        dom.at(r#"form[name="unit1"] input[name="device_owner"][value="Owner"]"#)
            .is_none()
    );
    assert!(
        dom.at(r#"form[name="unit1"] input[name="comments"][value="xyzzy"]"#)
            .is_none()
    );

    //but that they still exist
    let _ = dom.at(r#"form[name="unit1"] input[name="device_owner"][value]"#)
        .expect("failed to find empty owner");
    let _ = dom.at(r#"form[name="unit1"] input[name="comments"][value]"#)
        .expect("failed to find empty comments");
}
