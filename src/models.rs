use chrono;
use rocket;
use schema::devices;
use std;
use validator::{Validate, ValidationError};

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize, DbEnum,
)]
pub enum ReservationStatus {
    Available,
    Reserved,
}

impl Default for ReservationStatus {
    fn default() -> Self {
        ReservationStatus::Available
    }
}

impl std::ops::Not for ReservationStatus {
    type Output = ReservationStatus;

    fn not(self) -> Self::Output {
        match self {
            ReservationStatus::Available => ReservationStatus::Reserved,
            ReservationStatus::Reserved => ReservationStatus::Available,
        }
    }
}

use rocket::request::FromFormValue;
impl<'v> FromFormValue<'v> for ReservationStatus {
    type Error = &'v rocket::http::RawStr;

    fn from_form_value(v: &'v rocket::http::RawStr) -> Result<Self, Self::Error> {
        match v.to_lowercase().as_str() {
            "available" => Ok(ReservationStatus::Available),
            "reserved" => Ok(ReservationStatus::Reserved),
            _ => Err(v),
        }
    }
}

//deliberately not making this Copy
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Queryable, Serialize, Deserialize)]
pub struct Device {
    pub id: i32,
    pub device_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub device_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub device_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comments: Option<String>,
    pub reservation_status: ReservationStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[cfg_attr(
    feature = "cargo-clippy",
    allow(print_literal, suspicious_else_formatting)
)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Hash,
    Queryable,
    Serialize,
    Deserialize,
    FromForm,
    Validate,
)]
#[validate(schema(function = "validate_device_checkout"))]
pub struct DeviceUpdate {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub device_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comments: Option<String>,
    pub reservation_status: ReservationStatus,
}

fn validate_device_checkout(device: &DeviceUpdate) -> Result<(), ValidationError> {
    if device.reservation_status == ReservationStatus::Reserved {
        match device.device_owner {
            Some(ref owner) if !owner.trim().is_empty() => Ok(()),
            _ => {
                let mut e = ValidationError::new("reservation");
                e.message = Some("Please supply a username when reserving a device".into());
                Err(e)
            }
        }
    } else {
        Ok(())
    }
}

#[cfg_attr(
    feature = "cargo-clippy",
    allow(print_literal, suspicious_else_formatting)
)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Hash,
    Queryable,
    Serialize,
    Deserialize,
    FromForm,
    Validate,
)]
pub struct DeviceEdit {
    pub id: i32,
    #[validate(length(min = "1", message = "Device names cannot be empty"))]
    pub device_name: String,
    #[validate(url(message = "URL was invalid"))]
    pub device_url: String,
}

#[cfg_attr(
    feature = "cargo-clippy",
    allow(print_literal, suspicious_else_formatting)
)]
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Hash, Serialize, Deserialize, FromForm,
)]
pub struct DeviceDelete {
    pub id: i32,
}

#[cfg_attr(
    feature = "cargo-clippy",
    allow(print_literal, suspicious_else_formatting)
)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Hash,
    Queryable,
    Serialize,
    Deserialize,
    FromForm,
    Insertable,
    Validate,
)]
#[table_name = "devices"]
pub struct DeviceInsert {
    #[validate(length(min = "1", message = "Device names cannot be empty"))]
    pub device_name: String,
    #[validate(url(message = "URL was invalid"))]
    pub device_url: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_device_update_validation() {
        let mut device = DeviceUpdate {
            id: 3,
            device_owner: None,
            comments: None,
            reservation_status: ReservationStatus::Available,
        };
        assert!(device.validate().is_ok());
        device.reservation_status = ReservationStatus::Reserved;
        assert!(device.validate().is_err());
        device.device_owner = Some("toby".into());
        assert!(device.validate().is_ok());
    }

    #[test]
    fn test_device_insert_validation() {
        let mut device = DeviceInsert {
            device_name: "".into(),
            device_url: "".into(),
        };
        assert!(device.validate().is_err());
        device.device_name = "test".into();
        device.device_url = "http://test".into();
        assert!(device.validate().is_ok());
        device.device_name = "".into();
        assert!(device.validate().is_err());
    }

    #[test]
    fn test_device_edit_validation() {
        let mut device = DeviceEdit {
            id: 0,
            device_name: "".into(),
            device_url: "".into(),
        };
        assert!(device.validate().is_err());
        device.device_name = "test".into();
        device.device_url = "http://test".into();
        assert!(device.validate().is_ok());
        device.device_name = "".into();
        assert!(device.validate().is_err());
    }
}
