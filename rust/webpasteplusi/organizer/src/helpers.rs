use crate::models::*;

use diesel::{PgConnection, Connection, ConnectionResult,prelude::*};
use std::borrow::Borrow;
use base64::*;
pub mod parsers {
    use super::*;

    pub fn much_data_parse(
        data: &DumpCollector,
    ) -> (
        SubDomainsInsert,
        EndPointsInsert,
        EndPointInsert,
    ) {
        let sub: SubDomainsInsert = SubDomainsInsert {
            hostname: data.hostname.clone().unwrap(),
            is_root: None,
            ip: None,
            protocol: None,
            port: None,
            vhost: None,
            notes: None,

        };

        let endps = EndPointsInsert {
            list_type: "n".to_string(),
            href: data.link_from.clone().unwrap(),
            sid: 0,
        };

        let endp = EndPointInsert {
            value: data.path_only.clone().unwrap(),
            href: Option::from(data.link_from.clone().unwrap()),
            path_href: Option::from(data.full_path.clone().unwrap()),
            link_from: Option::from(data.path_only.clone().unwrap()),
            hitcount: 0,
            eid: 0,
        };


        (sub, endps, endp)
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

pub fn query_by_endpoint (conn: &PgConnection, endpoint: &str) {
    use crate::schema::dump_collector::dsl::*;
    let eid = encode(endpoint);
    let results: Vec<DumpCollector> = dump_collector
        .filter(endpoint_id.eq(eid))
        .get_results::<DumpCollector>(conn)
        .unwrap();
    debug!("Returned Data: {:?}", results);
}

pub fn organize_much_data (conn: &DbUtils) {
    let connection = conn.get_connection();
    let data = query_by_endpoint(connection, "/hostname/hostname_much_data");
}