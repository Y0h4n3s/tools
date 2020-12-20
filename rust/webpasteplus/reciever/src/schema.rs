table! {
    endpoint (id) {
        id -> Int4,
        value -> Nullable<Varchar>,
        params -> Nullable<Varchar>,
        eid -> Nullable<Int4>,
    }
}

table! {
    endpoints (id) {
        id -> Int4,
        date_added -> Timestamp,
        list_type -> Bpchar,
        rid -> Nullable<Int4>,
        sid -> Nullable<Int4>,
    }
}

table! {
    root_domains (id) {
        id -> Int4,
        hostname -> Nullable<Varchar>,
        ip -> Nullable<Varchar>,
        vhost -> Nullable<Bool>,
        date_added -> Timestamp,
        notes -> Nullable<Varchar>,
        protocol -> Nullable<Varchar>,
    }
}

table! {
    sub_domains (id) {
        id -> Int4,
        hostname -> Nullable<Varchar>,
        ip -> Nullable<Varchar>,
        vhost -> Nullable<Bool>,
        date_added -> Timestamp,
        notes -> Nullable<Varchar>,
        protocol -> Nullable<Varchar>,
        rid -> Nullable<Int4>,
    }
}

joinable!(endpoint -> endpoints (eid));
joinable!(endpoints -> sub_domains (sid));

allow_tables_to_appear_in_same_query!(
    endpoint,
    endpoints,
    root_domains,
    sub_domains,
);
