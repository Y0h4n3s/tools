use crate::schema::*;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::request_models::{MuchData, OwnLinks};

pub mod dbmodels {
    //TODO optimize with data types
    use super::*;


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
        pub path_only: String,
        pub full_link: String,
        pub extracted_from: String,
        pub params: String,
        pub port: i32,
    }
}


pub enum EndPoints {
    HostName {
        much_data: MuchData,
        own_links: OwnLinks
    }
}