use crate::schema::*;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Identifiable, Queryable, PartialEq, Debug, Deserialize, Insertable)]
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

#[derive(Deserialize, Debug, PartialEq, Queryable)]
pub struct DumpCollector {
    pub id: i32,
    pub hostname: Option<String>,
    pub full_path: Option<String>,
    pub protocol: Option<String>,
    pub path_only: Option<String>,
    pub full_params: Option<String>,
    pub href: Option<String>,
    pub path_href: Option<String>,
    pub link_from: Option<String>,
    pub ip: Option<String>,
    pub port: Option<i32>,
    pub endpoint_id: String
}

pub struct DumpCollectorAll {
    pub data: Vec<DumpCollector>,
}