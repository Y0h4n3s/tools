use super::schema::*;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod dbmodels {
    //TODO optimize with data types
    use super::*;



    #[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Insertable)]
    #[belongs_to(PortProtocol, foriegn_key = pid)]
    #[table_name = "sub_domains"]
    pub struct SubDomains {
        pub id: Option<i32>,
        pub hostname: String,
        pub is_root: Option<bool>,
        pub ip: Option<String>,
        pub vhost: Option<bool>,
        pub notes: Option<String>,
        pub protocol: Option<String>,
        pub port: Option<i32>,
    }
    #[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Insertable)]
    #[belongs_to(SubDomains, foriegn_key = sid)]
    #[table_name = "end_points"]
    pub struct EndPoints {
        pub id: Option<i32>,
        pub list_type: String,
        pub sid: i32,
    }

    #[derive(Deserialize, Debug, Insertable, Associations, PartialEq)]
    #[belongs_to(EndPoints, foriegn_key = eid)]
    #[table_name = "end_point"]
    pub struct EndPoint {
        pub value: String,
        pub href: Option<String>,
        pub path_href: Option<String>,
        pub link_from: Option<String>,
        pub hitcount: i32,
        pub eid: i32,
    }

    #[derive(Queryable, Identifiable, Debug, Associations, Deserialize)]
    #[belongs_to(EndPoints, foriegn_key = epid)]
    #[table_name = "params"]
    pub struct Params {
        pub id: Option<i32>,
        pub key: String,
        pub value: Option<String>,
        pub epid: i32,
    }

    pub struct MuchDataInsert {
        pub full_link: String,
        pub link_only: String,
        pub protocol: String,
        pub hostname: String,
        pub full_path: String,
        pub path_only: String,
        pub params: String,
        pub page_from: String,
    }

    #[derive(Deserialize, PartialEq, Associations, Insertable, Debug)]
    #[belongs_to(SubDomains, foriegn_key = sid)]
    #[table_name = "end_points"]
    pub struct EndPointsInsert {
        pub list_type: String,
        pub href: String,
        pub sid: i32,
    }
    #[derive(Deserialize, Debug, PartialEq, Insertable)]
    #[table_name = "sub_domains"]
    pub struct SubDomainsInsert {
        pub hostname: String,
        pub is_root: Option<bool>,
        pub ip: Option<String>,
        pub protocol: Option<String>,
        pub port: Option<i32>,
        pub vhost: Option<bool>,
        pub notes: Option<String>,
    }

    #[derive(Deserialize, Debug, PartialEq, Insertable)]
    #[table_name = "dump_collector"]
    pub struct DumpCollector {
        pub hostname: Option<String>,
        pub href: Option<String>,
        pub ip: Option<String>,
        pub protocol: Option<String>,
        pub port: Option<i32>,
        pub full_params: Option<String>,
        pub link_from: Option<String>,
        pub path_href: Option<String>,
        pub full_path: Option<String>,
        pub path_only: Option<String>,
        pub endpoint_id: String
    }

    #[derive(Deserialize, Debug, Insertable, Associations, PartialEq)]
    #[belongs_to(EndPoints, foriegn_key = eid)]
    #[table_name = "end_point"]
    pub struct EndPointInsert {
        pub value: String,
        pub href: Option<String>,
        pub path_href: Option<String>,
        pub link_from: Option<String>,
        pub hitcount: i32,
        pub eid: i32,
    }
    #[derive(Insertable, Debug, Deserialize, Associations, PartialEq)]
    #[belongs_to(EndPoint, foriegn_key = epid)]
    #[table_name = "params"]
    pub struct ParamsInsert {
        pub key: String,
        pub value: Option<String>,
        pub epid: i32,
    }
}

pub mod request_models {
    use super::*;
    #[derive(Deserialize, Serialize)]
    pub struct HostnameProtocol {
        pub data: Vec<HashMap<String, String>>,
        pub endpoint_id: String
    }
    #[derive(Deserialize, Serialize)]
    pub struct HostnameMuchData {
        pub data: Vec<MuchData>,
        pub endpoint_id: String
    }
    #[derive(Deserialize, Debug, Serialize)]
    pub struct MuchData {
        pub full_link: String,
        pub link_only: String,
        pub protocol: String,
        pub port: i32,
        pub hostname: String,
        pub full_path: String,
        pub path_only: String,
        pub params: String,
        pub page_from: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct HostnameOwnLinks {
        pub data: Vec<OwnLinks>,
        pub endpoint_id: String
    }
    #[derive(Deserialize, Debug)]
    pub struct OwnLinks {
        pub hostname: String,
        pub protocol: String,
        pub full_path: String,
        pub extracted_from: String,
        pub params: String,
    }
}
