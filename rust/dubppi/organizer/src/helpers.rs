use crate::models::*;

use diesel::{PgConnection, Connection, ConnectionResult, prelude::*, insert_into, update};
use std::borrow::Borrow;
use base64::*;
use diesel::r2d2::{PooledConnection, ConnectionManager, Pool};
use diesel::r2d2;
use std::process::exit;

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
    conn: Option<Pool<ConnectionManager<PgConnection>>>
}

impl DbUtils {
    pub fn new() -> DbUtils {

        DbUtils {
            db_creds: "".to_string(),
            conn: None
        }
    }



    pub fn connect(&mut self, db_creds: &String) -> &DbUtils{
        let manager = ConnectionManager::new(db_creds);
        let pool = r2d2::Pool::builder().build(manager).map_err(|e| {
            warn!("Failed To Connect To The Database: {:?}",e);
            debug!("Exiting...");
            exit(1);

        }).unwrap();

        self.conn = Option::from(pool);
        self
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.conn.as_ref()
            .unwrap()
            .get()
            .map_err(|e| {
                warn!("Failed To Get A Connection From The Database: {:?}",e);
                debug!("Exiting...");
                exit(1);
            })
            .unwrap()
    }
}
use diesel::expression_methods::ExpressionMethods;
pub fn query_all_by_endpoint (conn: &PgConnection, endpoint: &str, start_at: Option<i32>) -> Vec<DumpCollector> {
    use crate::schema::dump_collector::dsl::*;
    let eid = encode(endpoint);
    debug!("Endpoint Id: {}", eid);
    let results: Vec<DumpCollector> = dump_collector
        .filter(endpoint_id.eq(eid))
        .filter(id.gt(start_at.unwrap_or(1)))
        .get_results::<DumpCollector>(conn)
        .unwrap();
    debug!("Returned Data: {:?}", results);
    results
}

//insert into subdomains and return inserted element id or -1
pub fn insert_much_data_subdomains(dump: &DumpCollector, conn: &PgConnection) -> i32{


            use crate::schema::sub_domains::dsl::*;
            use super::*;
            let exists: Option<String> = sub_domains
                .filter(hostname.eq(dump.hostname.as_ref().unwrap_or((&"".to_string()))))
                .filter(protocol.eq(dump.protocol.as_ref().unwrap_or((&"".to_string()))))
                .filter(port.eq(dump.port.as_ref().unwrap_or(&443)))
                .select(hostname)
                .get_result::<Option<String>>(conn)
                .map_err(|e| {
                    debug!("An Entry For {} Doesn't Exist. Creating Now...", dump.hostname.as_ref().unwrap_or(&"".to_string()));
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
                    }).unwrap_or(-1);
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
        format!("{}{}{}{}",
                parsers::extract_origin(&data.href.as_ref().unwrap_or(&"".to_string())),
                &data.path_only.clone().unwrap_or("".to_string()),
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

pub fn organize_much_data (conn: &DbUtils, start_index: i32) {
    let connection = &conn.get_connection();
    let data = query_all_by_endpoint(connection, "/dom/much_data", Option::from(start_index));
    for dump in data {
        let subid = insert_much_data_subdomains(&dump, connection);
        if subid == -1 {
            warn!("Skipping {} Because SubDomains Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            println!("[-] Skipping {} Because SubDomains Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        let epsid = insert_much_data_endpoints(&dump, &subid,connection);
        if epsid == -1 {
            warn!("Skipping Endpoints At {} Because Endpoints Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            println!("[-] Skipping Endpoints At {} Because Endpoints Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        insert_much_data_endpoint(&dump, &epsid, connection);
    }
}


///
///
///own_links
///
///

fn insert_own_links_subdomains(dump: &DumpCollector, conn: &PgConnection) -> i32{
    use crate::schema::sub_domains::dsl::*;
    use super::*;
    let exists: Option<String> = sub_domains
        .filter(hostname.eq(dump.hostname.as_ref().unwrap_or((&"".to_string()))))
        .filter(protocol.eq(dump.protocol.as_ref().unwrap_or((&"".to_string()))))
        .filter(port.eq(dump.port.as_ref().unwrap_or(&443)))
        .select(hostname)
        .get_result::<Option<String>>(conn)
        .map_err(|e| {
            debug!("An Entry For {} Doesn't Exist. Creating Now...", dump.hostname.as_ref().unwrap_or(&"".to_string()));
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
            }).unwrap_or(-1);
    }
    debug!("Subid = {}", subid);
    subid
}

fn insert_own_links_endpoints(dump: &DumpCollector, conn: &PgConnection, subid: i32) -> i32 {
    use crate::schema::end_points::dsl::*;
    let mut epsid = -1;
    match end_points
        .filter((sid.eq(subid)))
        .filter((port.eq(&dump.port.unwrap().clone())))
        .filter((protocol.eq(&dump.protocol.clone().unwrap_or("".to_string()))))
        .filter(list_type.eq("n"))
        .select(id)
        .get_result::<i32>(conn)
        .map_err(|e| {
            debug!("An Error Occurred While Trying To Query The Endpoints Database For `{}`: {:?}", dump.hostname.as_ref().unwrap_or(&"".to_string()), e);
        }).unwrap_or(-1) {
         -1 => {
            let origin = parsers::extract_origin(dump.href.as_ref().unwrap_or(&"".to_string()));
            debug!("`{}` Doesn't Have An Endpoints Entry Associated Creating Now...", dump.hostname.as_ref().unwrap_or(&"".to_string()));
            let insertable_data = EndPointsInsert {
                list_type: "n".to_string(),
                href: origin,
                protocol: dump.protocol.clone(),
                port: dump.port,
                sid: subid
            };

            epsid = insert_into(end_points)
                .values(&insertable_data)
                .returning(id)
                .get_result::<i32>(conn)
                .map_err(|e| {
                    debug!("An Error Occurred While Trying To Insert `{}` Data Into Table `EndPoints`.", dump.hostname.as_ref().unwrap().clone());
                }).unwrap_or(-1);
        },
        other => {

        debug ! ("Endpoints Already Exists At `{}` Adding Endpoint Data Under That Entry...", other);
        epsid = other;
        }
    }
    epsid
}

fn insert_own_links_endpoint(dump: &DumpCollector, conn: &PgConnection, epsid: i32) {
    use crate::schema::end_point::dsl::*;
    let mut ful =  dump.path_only.clone().unwrap_or("".to_string());
    ful.push_str(&dump.full_params.clone().unwrap_or("".to_string()));
    let val = encode(
        format!("{}{}{}{}",
                parsers::extract_origin(&dump.href.as_ref().unwrap_or(&"".to_string())),
                &dump.path_only.clone().unwrap_or("".to_string()),
                ful.clone(),
                &dump.full_params.clone().unwrap_or("".to_string()),
        ));

    let exists = end_point
        .filter(value.eq(val.clone()))
        .select(id)
        .get_result::<i32>(conn)
        .map_err(|e| {
            debug!("An Error Occurred While Trying To Query The Endpoint Database For `{}` Data: {:?}", dump.hostname.as_ref().unwrap_or(&"".to_string()), e);
        }).unwrap_or(-1);
    if exists == -1 {
        debug!("Endpoint {} Doesn't Have An Endpoints Entry Associated Creating Now...", dump.hostname.as_ref().unwrap_or(&"".to_string()));

        let insertable_data = EndPointInsert {
            value: val.clone(),
            href: Option::from(parsers::extract_origin(&dump.href.as_ref().unwrap_or(&"".to_string()))),
            path_only: dump.path_only.clone(),
            link_from: dump.link_from.clone(),
            hitcount: 1,
            full_path: Option::from(ful.clone()),
            params: dump.full_params.clone(),
            eid: epsid
        };

        insert_into(end_point)
            .values(&insertable_data)
            .execute(conn)
            .map_err(|e| {
                debug!("An Error Occurred While Trying To Insert `{}` Data Into Table `EndPoint`: {:?}", dump.hostname.as_ref().unwrap().clone(),e);
            }).unwrap_or_default();
    } else {
        debug!("Endpoint Already Exists Incrementing Hitcount");
        let target = end_point.filter(id.eq(exists));
        update(target)
            .set(hitcount.eq(hitcount + 1))
            .execute(conn)
            .map_err(|e| {
                debug!("An Error Occurred While Trying To Increment Hitcount for `{}`", dump.path_only.as_ref().unwrap().clone());
            }).unwrap_or_default();
    }
}

pub fn organize_own_links (conn: &DbUtils, start_index: i32) {
    let connection = &conn.get_connection();
    let data = query_all_by_endpoint(connection, "/dom/own_links", Option::from(start_index));
    for dump in data {
        let subid = insert_own_links_subdomains(&dump, connection);
        if subid == -1 {
            warn!("Skipping {} Because SubDomains Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            println!("[-] Skipping {} Because SubDomains Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        let epsid = insert_own_links_endpoints(&dump, connection, subid);
        if epsid == -1 {
            warn!("Skipping Endpoints At {} Because Endpoints Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            println!("[-] Skipping Endpoints At {} Because Endpoints Insert Was Unsuccessful", dump.href.as_ref().unwrap_or(&"".to_string()));
            continue
        }
        insert_own_links_endpoint(&dump, connection, epsid);
    }
}

///
///
/// dom_xss
///
///

pub fn organize_dom_xss(connection: &PgConnection, start_index: i32) {
    use crate::schema::dom_xss::dsl::*;
    let data = query_all_by_endpoint(connection, "/dom/dom_xss", Option::from(start_index));
    for dump in &data {
        let mut insertable_data = DomXssInsert {
            kind: "".to_string(),
            link_from: dump.link_from.as_ref().unwrap_or(&"".to_string()).clone(),
            hostname: dump.hostname.as_ref().unwrap_or(&"".to_string()).clone(),
            value: "".to_string()
        };
        match &dump.path_href {
            Some(val) => {
                insertable_data.kind = "sink".to_string();
                insertable_data.value = val.clone();

            }
            None => {
                insertable_data.kind = "source".to_string();
                insertable_data.value = dump.full_params.as_ref().unwrap_or(&"".to_string()).clone();
            }
        }
        insert_into(dom_xss)
            .values(&insertable_data)
            .execute(connection)
            .map_err(|e| {
                warn!("Error Inserting To Dom_Xss Table");
            }).unwrap_or_default();
    }
}


pub fn update_last_processed(connection: &PgConnection) {
    use crate::schema::dump_collector::dsl::*;
    let all = dump_collector
        .select(id)
        .get_results::<i32>(connection)
        .map_err(|e| {
        debug!("Couldn't Save Current State ")
    }).unwrap();
    let last_index = Option::from(all.get(all.len() - 1).unwrap().to_string());
    {
        use crate::schema::configs::dsl::*;
        let last = configs
            .filter(key.eq("last_processed_dump_id"))
            .select(value)
            .get_result::<Option<String>>(connection)
            .unwrap_or(None);
        match last {
            Some(val) => {
                update(configs.filter(key.eq("last_processed_dump_id")))
                    .set(value.eq(last_index))
                    .execute(connection)
                    .map_err(|e| {
                        debug!("Couldn't Save Current State ")
                    })
                    .unwrap();
            }
            None => {
                insert_into(configs)
                    .values(ConfigsInsert {
                        key: "last_processed_dump_id".to_string(),
                        value: last_index
                    });
            }
        }

    };
}