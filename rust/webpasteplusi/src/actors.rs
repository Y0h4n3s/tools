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
    ) {
        use schema::sub_domains::dsl::*;
        let data_json = serde_json::to_string(&data.data).unwrap();
    }

    pub fn insert_hostname_much_data(
        data: &HostnameMuchData,
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
    ) {
        use schema::sub_domains::dsl::*;
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
                path_only: Option::from(single.path_only)
            };
            use crate::schema::dump_collector::dsl::*;
            insert_into(dump_collector)
                .values(&insertable_data)
                .execute(conn).map_err(|e| {
                warn!("Error Inserting Data To Dump: {:?}", insertable_data)
            }).unwrap();


        }
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
