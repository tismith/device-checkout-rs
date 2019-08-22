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
        pool_id -> Integer,
    }
}

table! {
    pools (id) {
        id -> Integer,
        pool_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(devices -> pools (pool_id));

allow_tables_to_appear_in_same_query!(devices, pools,);
