use crate::models::dbmodels::*;
use crate::models::request_models::*;
use crate::schema;
use crate::helpers::matchers::*;
use diesel::r2d2::*;
use diesel::PgConnection;
use diesel::*;

pub mod db_actors {
    use super::*;
    use futures::TryStreamExt;

    pub fn insert_dom_xss_sources(
        data: &DomXssSources,
        conn: &PooledConnection<ConnectionManager<PgConnection>>
    ) -> bool{
        use crate::schema::dump_collector::dsl::*;

        let my_endpoint_id = &data.endpoint_id;
        for sink in &data.data {

            let insertable_data = DumpCollector {
                hostname: Option::from(sink.hostname.clone()),
                href: None,
                ip: None,
                protocol: None,
                port: None,
                full_params: Option::from(sink.source.clone()),
                link_from: Option::from(sink.link_from.clone()),
                path_href: None,
                full_path: None,
                path_only: None,
                endpoint_id: "".to_string()
            };
            insert_into(dump_collector)
                .values(&insertable_data)
                .execute(conn)
                .map_err(|e| {
                    warn!("Error Inserting Data To Dump: {:?}", insertable_data);
                    warn!("Error: {:?}", e);
                    return false
                }).unwrap();
        }

        true
    }


    pub fn insert_dom_xss_sinks(
        data: &DomXssSinks,
        conn: &PooledConnection<ConnectionManager<PgConnection>>
    ) -> bool{
        use crate::schema::dump_collector::dsl::*;

        let my_endpoint_id = &data.endpoint_id;
        for sink in &data.data {

            let insertable_data = DumpCollector {
                hostname: Option::from(sink.hostname.clone()),
                href: None,
                ip: None,
                protocol: None,
                port: None,
                full_params: None,
                link_from: Option::from(sink.link_from.clone()),
                path_href: None,
                full_path: Option::from(sink.sink.clone()),
                path_only: None,
                endpoint_id: "".to_string()
            };
            insert_into(dump_collector)
                .values(&insertable_data)
                .execute(conn)
                .map_err(|e| {
                    warn!("Error Inserting Data To Dump: {:?}", insertable_data);
                    warn!("Error: {:?}", e);
                    return false
                }).unwrap();
        }

        true
    }


    pub fn insert_hostname_own_links(
        data: &DomOwnLinks,
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
        root_domain: Option<String>
    ) -> bool {
        let mut root = ".".to_string();
        if root_domain.is_some() {
            root = root_domain.unwrap();
        }
        use crate::schema::dump_collector::dsl::*;
        let my_endpoint_id = &data.endpoint_id;
        for own_links in &data.data {
            if domain_matches_by_hostname(&own_links.hostname, &root) {
                debug!("Domain {} Matches Regex {}", &own_links.hostname, &root);
                let insertable_data = DumpCollector {
                    hostname: Option::from(own_links.hostname.clone()),
                    href: Option::from(own_links.full_link.clone()),
                    ip: None,
                    protocol: Option::from(own_links.protocol.clone()),
                    port: Option::from(own_links.port.clone()),
                    full_params: Option::from(own_links.params.clone()),
                    link_from: Option::from(own_links.extracted_from.clone()),
                    path_href: None,
                    full_path: None,
                    path_only: Option::from(own_links.path_only.clone()),
                    endpoint_id: my_endpoint_id.clone()
                };
                insert_into(dump_collector)
                    .values(&insertable_data)
                    .execute(conn)
                    .map_err(|e| {
                        warn!("Error Inserting Data To Dump: {:?}", insertable_data);
                        warn!("Error: {:?}", e);
                        return false
                    }).unwrap();
            }
        }
        true

    }

    pub fn insert_hostname_much_data(
        data: &DomMuchData,
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
        root_domain: Option<String>
    ) -> bool {
        let mut root = ".".to_string();
        if root_domain.is_some() {
            root = root_domain.unwrap();
        }
        use crate::schema::dump_collector::dsl::*;
        let my_endpoint_id = &data.endpoint_id;
        let data_json = serde_json::to_string(&data.data).unwrap();
        let much_data = serde_json::from_str::<Vec<MuchData>>(&data_json).unwrap();
        //debug!("Extracted Data: {:?}", much_data);
        for single in much_data {
            if domain_matches_by_hostname(&single.hostname, &root) {
                debug!("Domain {} Matches Regex {}", &single.hostname, &root);
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
                    warn!("Error: {:?}", e);
                    return false
                }).unwrap();
            }
        }
        true
    }



}
