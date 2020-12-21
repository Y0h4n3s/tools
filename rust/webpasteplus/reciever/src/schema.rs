table! {
    end_point (id) {
        id -> Int4,
        value -> Nullable<Text>,
        params -> Nullable<Text>,
        hitcount -> Int4,
        eid -> Nullable<Int4>,
    }
}

table! {
    end_points (id) {
        id -> Int4,
        date_added -> Timestamp,
        list_type -> Bpchar,
        sid -> Nullable<Text>,
    }
}

table! {
    sub_domains (id) {
        id -> Int4,
        hostname -> Nullable<Text>,
        is_root -> Nullable<Bool>,
        ip -> Nullable<Text>,
        vhost -> Nullable<Bool>,
        date_added -> Timestamp,
        notes -> Nullable<Text>,
        protocol -> Nullable<Text>,
    }
}

joinable!(end_point -> end_points (eid));

allow_tables_to_appear_in_same_query!(
    end_point,
    end_points,
    sub_domains,
);
