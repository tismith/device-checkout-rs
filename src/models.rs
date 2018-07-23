use chrono;
use rocket;
use schema::devices;
use std;

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
)]
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
)]
pub struct DeviceEdit {
    pub id: i32,
    pub device_name: String,
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
)]
#[table_name = "devices"]
pub struct DeviceInsert {
    pub device_name: String,
    pub device_url: String,
}
