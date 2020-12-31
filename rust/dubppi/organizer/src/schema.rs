table! {
    config (id) {
        id -> Int4,
        key -> Text,
        value -> Nullable<Text>,
    }
}

table! {
    configs (id) {
        id -> Int4,
        key -> Text,
        value -> Nullable<Text>,
    }
}

table! {
    dom_xss (id) {
        id -> Int4,
        kind -> Text,
        link_from -> Nullable<Text>,
        hostname -> Nullable<Text>,
        value -> Text,
    }
}

table! {
    dump_collector (id) {
        id -> Int4,
        hostname -> Nullable<Text>,
        full_path -> Nullable<Text>,
        protocol -> Nullable<Text>,
        path_only -> Nullable<Text>,
        full_params -> Nullable<Text>,
        href -> Nullable<Text>,
        path_href -> Nullable<Text>,
        link_from -> Nullable<Text>,
        ip -> Nullable<Text>,
        port -> Nullable<Int4>,
        endpoint_id -> Text,
    }
}

table! {
    end_point (id) {
        id -> Int4,
        value -> Nullable<Text>,
        href -> Nullable<Text>,
        path_only -> Nullable<Text>,
        link_from -> Nullable<Text>,
        hitcount -> Int4,
        full_path -> Nullable<Text>,
        params -> Nullable<Text>,
        eid -> Nullable<Int4>,
    }
}

table! {
    end_points (id) {
        id -> Int4,
        date_added -> Timestamp,
        list_type -> Bpchar,
        href -> Nullable<Text>,
        port -> Nullable<Int4>,
        protocol -> Nullable<Text>,
        sid -> Nullable<Int4>,
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
joinable!(end_points -> sub_domains (sid));

allow_tables_to_appear_in_same_query!(
    config,
    configs,
    dom_xss,
    dump_collector,
    end_point,
    end_points,
    sub_domains,
);
