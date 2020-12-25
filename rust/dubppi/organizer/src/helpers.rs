use crate::models::*;

use diesel::{PgConnection, Connection, ConnectionResult, prelude::*, insert_into, update};
use std::borrow::Borrow;
use base64::*;
pub mod parsers {
    use super::*;

    pub fn extract_origin(href: &str) -> String {
        let matcher = regex::Regex::new("((?:https?|ftp|file|ssh)://[^/]*)").expect("Error Parsing Origin Extraction Regex");
        let matches = matcher.captures(href).expect("Error Capturing Matches");
        //info!("Origin Is: {}", matches[1].to_string());
        matches[1].to_string()
    }
    }


pub struct DbUtils {
    db_creds: String,
    conn: Option<PgConnection>
}

impl DbUtils {
    pub fn new() -> DbUtils {

        DbUtils {
            db_creds: "".to_string(),
            conn: None
        }
    }

    pub fn check_connect(&self, db_creds: &String) -> ConnectionResult<PgConnection> {
        PgConnection::establish(db_creds)
    }

    pub fn connect(&mut self, db_creds: &String) -> &DbUtils{
        self.conn = Option::from(PgConnection::establish(db_creds).unwrap());
        self
    }

    pub fn get_connection(&self) -> &PgConnection {
        &self.conn.as_ref().unwrap()
    }
}

//insert into subdomains and return inserted element id or -1
fn insert_much_data_subdomains(dump: &DumpCollector, conn: &PgConnection) -> i32{


            use crate::schema::sub_domains::dsl::*;
            use super::*;
            let exists: Option<String> = sub_domains
                .filter(hostname.eq(dump.hostname.as_ref().unwrap_or((&"".to_string()))))
                .filter(protocol.eq(dump.protocol.as_ref().unwrap_or((&"".to_string()))))
                .filter(port.eq(dump.port.as_ref().unwrap_or(&443)))
                .select(hostname)
                .get_result::<Option<String>>(conn)
                .map_err(|e| {
                    debug!("An Error Occurred While Trying To Query The Database For `{}`: {:?}", dump.hostname.as_ref().unwrap_or(&"".to_string()), e);
                }).unwrap_or(None);
            let mut subid = -1;
            if exists.is_none() {
                let insertable_data = SubDomainsInsert {
                    hostname: dump.hostname.as_ref().unwrap().clone(),
                    is_root: None,
                    ip: None,
                    protocol: dump.protocol.clone(),
                    port: dump.port,
                    vhost: None,
                    notes: None
                };
                subid = insert_into(sub_domains)
                    .values(&insertable_data)
                    .returning(id)
                    .get_result::<i32>(conn)
                    .map_err(|e| {
                        debug!("An Error Occurred While Trying To Insert {} Data Into SubDomains Table.", dump.hostname.as_ref().unwrap().clone());
                    }).unwrap_or(-1);

            } else {
                subid = sub_domains
                    .filter(hostname.eq(dump.hostname.as_ref().unwrap_or((&"".to_string()))))
                    .filter(protocol.eq(dump.protocol.as_ref().unwrap_or((&"".to_string()))))
                    .filter(port.eq(dump.port.as_ref().unwrap_or(&443)))
                    .select(id)
                    .get_result::<i32>(conn)
                    .map_err(|e| {
                        debug!("An Error Occurred While Trying To Query The Database For `{}`: {:?}", dump.hostname.as_ref().unwrap_or(&"".to_string()), e);
                    }).unwrap_or(0);
            }
            debug!("Subid = {}", subid);
            subid
        }

fn insert_much_data_endpoints (data: &DumpCollector, subid: &i32, conn: &PgConnection) -> i32{
    use crate::schema::end_points::dsl::*;
    let exists = end_points
        .filter((sid.eq(subid)))
        .filter((port.eq(&data.port.unwrap().clone())))
        .filter((protocol.eq(&data.protocol.clone().unwrap_or("".to_string()))))
        .filter(list_type.eq("n"))
        .select(id)
        .get_result::<i32>(conn)
        .map_err(|e| {
            debug!("An Error Occurred While Trying To Query The Endpoints Database For `{}`: {:?}", data.hostname.as_ref().unwrap_or(&"".to_string()), e);
        }).unwrap_or(-1);
    let mut epsid = -1;
    if exists == -1 {
        let origin = parsers::extract_origin(data.href.as_ref().unwrap_or(&"".to_string()));
        debug!("`{}` Doesn't Have An Endpoints Entry Associated Creating Now...", data.hostname.as_ref().unwrap_or(&"".to_string()));
        let insertable_data = EndPointsInsert {
            list_type: "n".to_string(),
            href: origin,
            protocol: data.protocol.clone(),
            port: data.port,
            sid: *subid
        };

         epsid = insert_into(end_points)
            .values(&insertable_data)
            .returning(id)
            .get_result::<i32>(conn)
            .map_err(|e| {
                debug!("An Error Occurred While Trying To Insert `{}` Data Into Table `EndPoints`.", data.hostname.as_ref().unwrap().clone());
            }).unwrap_or(-1);
    } else {

        debug!("Endpoints Already Exists At `{}` Adding Endpoint Data Under That Entry...", exists);
        epsid = exists;
    }
    epsid

}

fn insert_much_data_endpoint(data: &DumpCollector, epsid: &i32, conn: &PgConnection) {
    use crate::schema::end_point::dsl::*;
    let val = encode(
        format!("{}{}{}{}{}",
                parsers::extract_origin(&data.href.as_ref().unwrap_or(&"".to_string())),
                &data.path_only.clone().unwrap_or("".to_string()),
                &data.link_from.clone().unwrap_or("".to_string()),
                &data.full_path.clone().unwrap_or("".to_string()),
                &data.full_params.clone().unwrap_or("".to_string()),
        ));

    let exists = end_point
        .filter(value.eq(val.clone()))
        .select(id)
        .get_result::<i32>(conn)
        .map_err(|e| {
            debug!("An Error Occurred While Trying To Query The Endpoint Database For `{}` Data: {:?}", data.hostname.as_ref().unwrap_or(&"".to_string()), e);
        }).unwrap_or(-1);
    if exists == -1 {
        debug!("Endpoint {} Doesn't Have An Endpoints Entry Associated Creating Now...", data.hostname.as_ref().unwrap_or(&"".to_string()));

        let insertable_data = EndPointInsert {
            value: val.clone(),
            href: Option::from(parsers::extract_origin(&data.href.as_ref().unwrap_or(&"".to_string()))),
            path_only: data.path_only.clone(),
            link_from: data.link_from.clone(),
            hitcount: 1,
            full_path: data.full_path.clone(),
            params: data.full_params.clone(),
            eid: *epsid
        };

        insert_into(end_point)
            .values(&insertable_data)
            .execute(conn)
            .map_err(|e| {
                debug!("An Error Occurred While Trying To Insert `{}` Data Into Table `EndPoint`: {:?}", data.hostname.as_ref().unwrap().clone(),e);
            }).unwrap_or_default();
    } else {
        debug!("Endpoint Already Exists Incrementing Hitcount");
        let target = end_point.filter(id.eq(exists));
        update(target)
            .set(hitcount.eq(hitcount + 1))
            .execute(conn)
            .map_err(|e| {
                debug!("An Error Occurred While Trying To Increment Hitcount for `{}`", data.path_only.as_ref().unwrap().clone());
            }).unwrap_or_default();
    }
}

pub fn query_by_endpoint (conn: &PgConnection, endpoint: &str) -> Vec<DumpCollector> {
    use crate::schema::dump_collector::dsl::*;
    let eid = encode(endpoint);
    debug!("Endpoint Id: {}", eid);
    let results: Vec<DumpCollector> = dump_collector
        .filter(endpoint_id.eq(eid))
        .get_results::<DumpCollector>(conn)
        .unwrap();
    debug!("Returned Data: {:?}", results);
    results
}

pub fn organize_much_data (conn: &DbUtils) {
    let connection = conn.get_connection();
    let data = query_by_endpoint(connection, "/hostname/hostname_much_data");
    for dump in data {
        let subid = insert_much_data_subdomains(&dump, connection);
        if subid == -1 {
            warn!("Skipping {} Because Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        let epsid = insert_much_data_endpoints(&dump, &subid,connection);
        if epsid == -1 {
            warn!("Skipping Endpoints At {} Because Endpoints Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        insert_much_data_endpoint(&dump, &epsid, connection);
    }
}