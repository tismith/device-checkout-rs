use chrono;

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

//deliberately not making this Copy
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Hash, Queryable, Serialize,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}
