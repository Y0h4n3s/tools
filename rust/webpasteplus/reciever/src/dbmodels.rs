
use crate::schema::*;
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Insertable, Deserialize, Queryable, QueryableByName)]
#[table_name="sub_domains"]
pub struct SubDomainInsert {
    #[sql_type = "String"]
    pub hostname: String,
}

#[derive(Identifiable,Queryable, PartialEq, Debug, QueryableByName)]
#[table_name="sub_domains"]
pub struct SubDomain {
    pub id: i32,
    pub hostname: String,
    pub is_root: Option<bool>,
    pub ip: Option<String>,
    pub vhost: Option<bool>,
    pub date_added: SystemTime,
    pub notes: Option<String>,
    pub protocol: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Insertable)]
#[belongs_to(SubDomain, foriegn_key = sid)]
#[table_name="end_points"]
pub struct EndPoints{
    pub id: Option<i32>,
    pub list_type: String,
    pub sid: String,
}

#[derive( Associations, PartialEq, Debug, Deserialize, Insertable)]
#[belongs_to(SubDomain, foriegn_key = sid)]
#[table_name="end_points"]
pub struct EndPointsInsert{
    pub list_type: String,
    pub sid: String,
}
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Insertable)]
#[belongs_to(EndPoints, foriegn_key = eid)]
#[table_name="end_point"]
pub struct EndPoint {
    pub id: Option<i32>,
    pub value : Option<String>,
    pub params: Option<String>,
    pub hitcount: Option<i32>,
    pub eid: i32,
}

#[derive(Associations, PartialEq, Debug, Deserialize, Insertable)]
#[belongs_to(EndPoints, foriegn_key = eid)]
#[table_name="end_point"]
pub struct EndPointInsert {
    pub value : Option<String>,
    pub params: Option<String>,
    pub hitcount: i32,
    pub eid: i32,
}


pub trait Jsoner<T> {
    fn jsonify(&self) -> serde_json::Result<T>;
}

pub trait JsonerWithFK<T> {
    fn jsonify(&self, fk: i32) -> serde_json::Result<T>;
}
pub trait JsonerWithFKString<T> {
    fn jsonify(&self, fk: String) -> serde_json::Result<T>;
}

impl Jsoner<SubDomainInsert> for SubDomainInsert{
    fn jsonify(&self) -> serde_json::Result<SubDomainInsert> {
        let mut json = format!(r#"{{"hostname": "{}", "ip": null, "vhost": null, "notes": null, "is_root":null}}"#, self.hostname);
        //println!("canceled?? {}",json);
        serde_json::from_str::<SubDomainInsert>(&json)
    }
}

impl JsonerWithFKString<EndPointsInsert> for EndPointsInsert{
    fn jsonify(&self, fk: String) -> serde_json::Result<EndPointsInsert> {
        let mut json = format!(r#"{{"list_type": "s", "sid": "{}"}}"#, fk);
        //println!("canceled?? {}",json);
        serde_json::from_str::<EndPointsInsert>(&json)
    }
}

impl JsonerWithFK<EndPointInsert> for EndPointInsert{
    fn jsonify(&self, fk: i32) -> serde_json::Result<EndPointInsert> {
        let mut json = format!(r#"{{"value": "{}", "params":null, "hitcount": {},  "eid": {}}}"#, self.value.as_ref().unwrap_or(&"".to_string()), self.hitcount, fk);
        //println!("canceled?? {}",json);
        serde_json::from_str::<EndPointInsert>(&json)
    }
}

