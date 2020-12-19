table! {
    endpoint (epid) {
        epid -> Int4,
        value -> Nullable<Varchar>,
        eid -> Nullable<Int4>,
    }
}

table! {
    endpoints (eid) {
        eid -> Int4,
        date_added -> Timestamp,
        list_type -> Bpchar,
        rid -> Nullable<Int4>,
        sid -> Nullable<Int4>,
    }
}

table! {
    params (pmid) {
        pmid -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        parameter_name -> Nullable<Varchar>,
        epid -> Nullable<Int4>,
    }
}

table! {
    port_protocol (pid) {
        pid -> Int4,
        port -> Int4,
        protocol -> Varchar,
    }
}

table! {
    root_domains (rid) {
        rid -> Int4,
        hostname -> Nullable<Varchar>,
        ip -> Nullable<Varchar>,
        vhost -> Nullable<Bool>,
        date_added -> Timestamp,
        notes -> Nullable<Varchar>,
        pid -> Nullable<Int4>,
    }
}

table! {
    sub_domains (sid) {
        sid -> Int4,
        hostname -> Nullable<Varchar>,
        ip -> Nullable<Varchar>,
        vhost -> Nullable<Bool>,
        date_added -> Timestamp,
        notes -> Nullable<Varchar>,
        pid -> Nullable<Int4>,
        rid -> Nullable<Int4>,
    }
}

joinable!(endpoint -> endpoints (eid));
joinable!(endpoints -> sub_domains (sid));
joinable!(params -> endpoint (epid));
joinable!(root_domains -> port_protocol (pid));
joinable!(sub_domains -> port_protocol (pid));

allow_tables_to_appear_in_same_query!(
    endpoint,
    endpoints,
    params,
    port_protocol,
    root_domains,
    sub_domains,
);
