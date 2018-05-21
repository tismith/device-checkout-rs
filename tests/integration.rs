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
    let _ = dom.at(r#"form[name="reserve-1"] a[href="http://unit1"]"#)
        .expect("failed to find unit1");
    let _ = dom.at(r#"form[name="reserve-2"] a[href="http://unit2"]"#)
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
    let _ = dom.at(r#"form[name="edit-1"] input[name="device_url"][value="http://unit1"]"#)
        .expect("failed to find unit1");
    let _ = dom.at(r#"form[name="edit-2"] input[name="device_url"][value="http://unit2"]"#)
        .expect("failed to find unit2");
}

fn get_cookies(response: &rocket::local::LocalResponse) -> Vec<rocket::http::Cookie<'static>> {
    let mut cookies = Vec::new();
    for header in response.headers().get("Set-Cookie") {
        if let Ok(cookie) = rocket::http::Cookie::parse_encoded(header) {
            cookies.push(cookie.into_owned());
        }
    }
    cookies
}

fn get_redirect(response: &rocket::local::LocalResponse) -> Option<String> {
    if response.status() == rocket::http::Status::SeeOther {
        response
            .headers()
            .get("Location")
            .next()
            .map(|loc| loc.to_string())
    } else {
        None
    }
}

fn follow_redirect<'a>(
    client: &'a rocket::local::Client,
    response: &rocket::local::LocalResponse,
) -> Option<rocket::local::LocalResponse<'a>> {
    let cookies = get_cookies(&response);
    let location = match get_redirect(&response) {
        Some(l) => l,
        None => return None,
    };

    //manually follow the redirection with a new client
    let mut request = client.get(location);

    for cookie in cookies {
        request = request.cookie(cookie);
    }

    Some(request.dispatch())
}

#[test]
fn test_html_post_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);

    let body = response.body_string().unwrap();
    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form[name="reserve-1"] input[name="device_owner"][value="Owner"]"#)
        .expect("failed to find owner");

    let _ = dom.at(r#"form[name="reserve-1"] input[name="reservation_status"][value="Reserved"]"#)
        .expect("failed to find reservation status");

    let _ = dom.at(r#"form[name="reserve-1"] input[name="comments"][value="xyzzy"]"#)
        .expect("failed to find comments");
}

#[test]
fn test_html_edit_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/editDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_name=testunit&device_url=testurl&save=SAVE"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form[name="edit-1"] input[name="device_name"][value="testunit"]"#)
        .expect("failed to find edited device name");

    let _ = dom.at(r#"form[name="edit-1"] input[name="device_url"][value="testurl"]"#)
        .expect("failed to find edited device url");
}

#[test]
fn test_html_edit_devices_delete() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/deleteDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_name=testunit&device_url=testurl&delete=DELETE"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    assert!(dom.at(r#"form[name="edit-1"]"#).is_none());
}

#[test]
fn test_html_add_devices() {
    let file = tempfile::NamedTempFile::new().expect("creating tempfile");
    let mut config = utils::types::Settings::new();
    config.database_url = file.path().to_string_lossy().to_owned().to_string();

    database::run_migrations(&config).expect("running migrations");

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    let response = client
        .post("/addDevices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"device_name=testunit&device_url=testurl&add=ADD"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    let _ = dom.at(r#"form input[name="device_url"][value="testurl"]"#)
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

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    //reserve unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);
    let _ = dom.at(r#"form[name="reserve-1"] input[name="reservation_status"][value="Reserved"]"#)
        .expect("failed to find reservation status");

    //reserve unit2
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner2&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();

    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#error_message"#)
        .expect("failed to find error message");
    assert!(dom.at(r#"#success_message"#).is_none());

    let _ = dom.at(r#"form[name="reserve-1"] input[name="device_owner"][value="Owner"]"#)
        .expect("failed to find owner");
    assert!(
        dom.at(r#"form[name="reserve-1"] input[name="device_owner"][value="Owner2"]"#)
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

    let rocket = routes::rocket(config).expect("creating rocket instance");
    let client = rocket::local::Client::new(rocket).expect("valid rocket instance");

    //reserve unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&device_owner=Owner&comments=xyzzy&reservation_status=Available"#)
        .dispatch();

    let response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);

    //return unit1
    let response = client
        .post("/devices")
        .header(rocket::http::ContentType(rocket::http::MediaType::Form))
        .body(r#"id=1&reservation_status=Reserved"#)
        .dispatch();

    let mut response = follow_redirect(&client, &response).unwrap();
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.body_string().unwrap();

    let dom = victoria_dom::DOM::new(&body);

    let _ = dom.at(r#"#success_message"#)
        .expect("failed to find success message");
    assert!(dom.at(r#"#error_message"#).is_none());

    //test that the old values for the reservation are gone
    assert!(
        dom.at(r#"form[name="reserve-1"] input[name="device_owner"][value="Owner"]"#)
            .is_none()
    );
    assert!(
        dom.at(r#"form[name="reserve-1"] input[name="comments"][value="xyzzy"]"#)
            .is_none()
    );

    //but that they still exist
    let _ = dom.at(r#"form[name="reserve-1"] input[name="device_owner"][value]"#)
        .expect("failed to find empty owner");
    let _ = dom.at(r#"form[name="reserve-1"] input[name="comments"][value]"#)
        .expect("failed to find empty comments");
}
