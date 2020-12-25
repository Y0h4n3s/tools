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
