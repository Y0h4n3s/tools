table! {
    dns_names (id) {
        id -> Int4,
        a -> Nullable<Text>,
        mx -> Nullable<Text>,
        sid -> Nullable<Text>,
    }
}

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
    params (id) {
        id -> Int4,
        key -> Nullable<Text>,
        value -> Nullable<Text>,
        epid -> Nullable<Int4>,
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
        port -> Nullable<Int4>,
        protocol -> Nullable<Text>,
    }
}

joinable!(end_point -> end_points (eid));
joinable!(params -> end_point (epid));

allow_tables_to_appear_in_same_query!(
    dns_names,
    end_point,
    end_points,
    params,
    sub_domains,
);
