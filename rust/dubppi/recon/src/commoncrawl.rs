use std::process::exit;

use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures::StreamExt;
use reqwest::get;
use tokio::*;

use consumer::helpers::parsers::parse_chunk_into_dom_much_data;
use consumer::models::request_models::MuchData;

use crate::{AppConfig, ResponseHandler};

pub struct CommonCrawlUrls {
    pool: Pool<ConnectionManager<PgConnection>>,
    commoncrawl_config: CommonCrawlConfig,
}

impl CommonCrawlUrls {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, commoncrawl_config: CommonCrawlConfig) -> CommonCrawlUrls {
        CommonCrawlUrls { pool: pool, commoncrawl_config }
    }

    pub(crate) async fn start(&self) {
        use organizer::schema::sub_domains::dsl::*;
        let conn = self.pool.get();
        let conn = match conn {
            Ok(connection) => { connection }
            Err(_) => { /* maybe save to file .await*/exit(-1) }
        };
        let subs: std::result::Result<Vec<Option<String>>, diesel::result::Error> =
            sub_domains
                .select(hostname)
                .get_results::<Option<String>>(&conn);

        match subs {
            Ok(data) => {
                let subs = data
                    .into_iter()
                    .filter(|e| !e.as_ref().unwrap_or(&"".to_string()).eq(""))
                    .map(|e| e.as_ref().unwrap().clone())
                    .collect::<Vec<String>>();

                debug!("Querying CommonCrawl For: {:?}", &subs);

                let commoncrawl_data =
                    stream::iter(subs)
                        .map(|sub| {
                            async move {
                                let indexi = vec!["2020-50", "2020-45", "2020-40", "2020-34", "2020-29", "2020-24", "2020-16", "2020-10", "2020-5", "2019-51", "2019-47", "2019-43", "2019-39", "2019-35", "2019-30", "2019-26", "2019-22", "2019-18", "2019-13", "2019-09", "2019-04", "2018-51", "2018-47", "2018-43", "2018-39", "2018-34", "2018-30", "2018-26", "2018-22", "2018-17", "2018-13", "2018-09", "2018-05", "2017-51", "2017-47", "2017-51", "2017-43", "2017-39", "2017-34", "2017-30", "2017-26", "2017-22", "2017-17", "2017-13", "2017-09", "2017-04", "2016-50", "2016-44", "2016-40", "2016-36", "2016-30", "2016-26", "2016-22", "2016-18", "2016-07", "2015-48", "2015-40", "2015-35", "2015-32", "2015-27", "2015-22", "2015-18", "2015-14", "2015-11", "2015-06", "2014-52", "2014-49", "2014-42", "2014-41", "2014-35", "2014-23", "2014-15", "2014-10", "2013-48", "2013-20", "2012", "2008-2010", "2008-2009"];

                                let mut responses = Vec::new();
                                for index in indexi {
                                    let req = get(&format!("http://index.commoncrawl.org/CC-MAIN-{}-index?url=*.{}/*|{}/*|*{}*|{}&output=json&collapse=urlkey", index, sub, sub, sub, sub)).await;
                                    match req {
                                        Ok(response) => { responses.push(response.bytes().await); }
                                        Err(e) => { /*TODO implement saving failed requests*/ debug!("{:?}", e.to_string()) }
                                    }
                                }
                                responses
                            }
                        })
                        .buffered(self.commoncrawl_config.async_conns as usize);
                debug!("Commoncrawl Data: {:?}", &commoncrawl_data);

                commoncrawl_data.for_each(|res| async {
                    for i in res {
                        match i {
                            Ok(data) => {
                                self.handle(data.to_vec())
                            }
                            Err(err) => {
                                debug!("{:?}", err.to_string())
                            }
                        }
                    }
                }).await;
            }

            _ => {}
        }
    }
}


impl ResponseHandler for CommonCrawlUrls {
    fn handle(&self, data: Vec<u8>) {
        let s_data = String::from_utf8_lossy(&*data);
        let indiv: Vec<&str> = s_data.trim().split("\n").collect();
        let chunk: Vec<&str> = indiv.iter().map(|x| {
            let splitzee: Vec<&str> = x.split("\", \"").collect();
            match splitzee.get(2) {
                None => { /* who cares */ "" }
                Some(val) => {
                    match val.split(": \"").collect::<Vec<&str>>().get(1) {
                        None => { "" }
                        Some(l) => l
                    }
                }
            }
        }).collect();

        let chunk = chunk.join(" ");
        let mut to_insert: Vec<MuchData> = Vec::new();

        let val = parse_chunk_into_dom_much_data(&chunk);
        consumer::actors::db_actors::insert_dom_much_data(&val, &self.pool.get().unwrap(), Option::from(String::from(&self.commoncrawl_config.root_domain.clone())));
        debug!("Inserted :)")
    }
}


pub struct CommonCrawlConfig {
    pub root_domain: String,
    pub async_conns: i32,
}


impl From<AppConfig> for CommonCrawlConfig {
    fn from(app_config: AppConfig) -> Self {
        CommonCrawlConfig {
            root_domain: app_config.root_domain.unwrap_or(".".to_string()),
            async_conns: app_config.async_conns,
        }
    }
}