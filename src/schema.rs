table! {
    devices (id) {
        id -> Integer,
        device_name -> Text,
        device_url -> Nullable<Text>,
        device_owner -> Nullable<Text>,
        comments -> Nullable<Text>,
        reservation_status -> ::models::ReservationStatusMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        room_id -> Integer,
    }
}

table! {
    rooms (id) {
        id -> Integer,
        room_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(devices -> rooms (room_id));

allow_tables_to_appear_in_same_query!(devices, rooms,);
