use chrono;
use rocket;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
         DbEnum)]
pub enum ReservationStatus {
    Available,
    Reserved,
}

impl Default for ReservationStatus {
    fn default() -> Self {
        ReservationStatus::Available
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Queryable, Serialize,
         Deserialize)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Hash, Queryable, Serialize,
         Deserialize, FromForm)]
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
