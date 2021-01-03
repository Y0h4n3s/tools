use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures::future::join_all;
use futures::StreamExt;
use reqwest::{Error, get, Response};
use tokio::stream;

use consumer::helpers::parsers::*;

use crate::{AppConfig, ResponseHandler};
use crate::models::WaybackData;
use consumer::models::request_models::{MuchData, DomMuchData};
use diesel::query_dsl::InternalJoinDsl;

#[derive(Clone)]
pub struct WayBackUrls {
    pool: Pool<ConnectionManager<PgConnection>>,
    wayback_config: WaybackConfig,
}

impl WayBackUrls {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, wayback_config: WaybackConfig) -> WayBackUrls {
        debug!("Got the pool");
        WayBackUrls { pool: pool, wayback_config }
    }

    pub(crate) async fn start(&self) {
        debug!("getting subs");
        use organizer::schema::sub_domains::dsl::*;

        let subs: std::result::Result<Vec<Option<String>>, diesel::result::Error> = sub_domains.select(hostname)
            .get_results::<Option<String>>(&self.pool.get().unwrap());

        match subs {
            Ok(data) => {
                let subs: Vec<String> = data.into_iter().map(|e| e.unwrap()).collect();
                debug!("Subs: {:?}", &subs);
                let wayback_data =
                    stream::iter(subs)
                        .map(|sub| {
                            async move {
                                let req = get(&format!("http://web.archive.org/cdx/search/cdx?url=.*{}/*&output=json&collapse=urlkey", sub)).await;
                                match req {
                                    Ok(response) => { response.bytes().await }
                                    Err(e) => { /*TODO implement saving failed requests*/ Err(e) }
                                }
                            }
                        }).buffered(self.wayback_config.async_conns);
                debug!("Wayback Data: {:?}", &wayback_data);

                wayback_data.for_each(|res| async {
                    match res {
                        Ok(data) => {
                            self.handle(data.to_vec())
                        }
                        Err(err) => {
                            debug!("{:?}", err)
                        }
                    }
                }).await;
            }

            _ => {}
        }
    }
}

impl ResponseHandler for WayBackUrls {
    // TODO implement a cyclic barrier to optimize this
    fn handle(&self, data: Vec<u8>) {
        let s_data = String::from_utf8_lossy(&*data);
        let indiv: Vec<&str> = s_data.trim().split("\n").collect();
        let chunk: Vec<&str> = indiv.iter().map(|x|{
            let splitzee: Vec<&str> = x.split("\", \"").collect();
            match splitzee.get(2) {
                None => { /* who cares */ "" }
                Some(val) => {
                    val
                }
            }
        }).collect();
        let chunk = chunk.join(" ");
        let mut to_insert: Vec<MuchData> = Vec::new();
        parse_chunk_into_dom_much_data(&chunk);
        let val = DomMuchData {
            data: to_insert,
            endpoint_id: base_64_me("/dom/much_data")
        };
        consumer::actors::db_actors::insert_dom_much_data(&val, &self.pool.get().unwrap(), None);
        debug!("Inserted :)")
    }
}


#[derive(Clone, Debug)]
pub struct WaybackConfig {
    pub dbcreds: String,
    pub async_conns: usize,
}

impl From<AppConfig> for WaybackConfig {
    fn from(app_config: AppConfig) -> Self {
        WaybackConfig {
            dbcreds: app_config.dbcreds,
            async_conns: 5,
        }
    }
}

