use super::models::dbmodels::*;
use super::models::request_models::*;
use super::schema;
use super::helpers::parsers::*;
use diesel::r2d2::*;
use diesel::PgConnection;
use diesel::*;

pub mod db_actors {
    use super::*;

    pub fn insert_hostname_protocol(
        data: &HostnameProtocol,
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
    ) -> bool {
        use crate::schema::dump_collector::dsl::*;
        let my_endpoint_id = &data.endpoint_id;
        for hostproto in &data.data {
            let insertable_data = DumpCollector {
                hostname: Option::from(hostproto["hostname"].clone()),
                href: None,
                ip: None,
                protocol: Option::from(hostproto["protocol"].clone()),
                port: None,
                full_params: None,
                link_from: None,
                path_href: None,
                full_path: None,
                path_only: None,
                endpoint_id: my_endpoint_id.clone()
            };
            insert_into(dump_collector)
                .values(&insertable_data)
                .execute(conn)
                .map_err(|e| {
                warn!("Error Inserting Data To Dump: {:?}", e);
                    return false
                }).unwrap();
        }
        true

    }

    pub fn insert_hostname_much_data(
        data: &HostnameMuchData,
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
    ) -> bool {
        use crate::schema::dump_collector::dsl::*;
        let my_endpoint_id = &data.endpoint_id;
        let data_json = serde_json::to_string(&data.data).unwrap();
        let much_data = serde_json::from_str::<Vec<MuchData>>(&data_json).unwrap();
        debug!("Extracted Data: {:?}", much_data);
        let mut subs: Vec<SubDomainsInsert> = Vec::new();
        let mut endpss: Vec<EndPointsInsert> = Vec::new();
        let mut endps: Vec<EndPointInsert> = Vec::new();
        let mut paramss: Vec<Vec<ParamsInsert>> = Vec::new();
        for single in much_data {
            let insertable_data = DumpCollector {
                hostname: Option::from(single.hostname),
                href: Option::from(single.full_link),
                ip: None,
                protocol: Option::from(single.protocol),
                port: Option::from(single.port),
                full_params: Option::from(single.params),
                link_from: Option::from(single.page_from),
                path_href: Option::from(single.full_path),
                full_path: None,
                path_only: Option::from(single.path_only),
                endpoint_id: my_endpoint_id.clone()
            };

            insert_into(dump_collector)
                .values(&insertable_data)
                .execute(conn).map_err(|e| {
                warn!("Error Inserting Data To Dump: {:?}", insertable_data);
                return false
            }).unwrap();
        }
        true
    }

    pub(crate) fn check_sub_exists(
        sub: &SubDomains,
        port: i32,
        protocol: &String,
        conn: &PooledConnection<ConnectionManager<PgConnection>>
    ) -> (bool, i32) {


        (true, 2)
    }

}
