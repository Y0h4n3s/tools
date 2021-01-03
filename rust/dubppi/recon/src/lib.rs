#![allow(unused)]
#[macro_use] extern crate log;
extern crate pretty_env_logger;
mod thread_pool;
mod models;
mod wayback;
mod commoncrawl;
mod amass;

use std::fs::File;
use std::thread;
use std::time::Duration;
use dotenv::dotenv;
use reqwest::*;
use diesel::{PgConnection, Connection, QueryDsl, r2d2};
use wayback::*;
use crate::models::WaybackData;
use tokio::*;
use futures::future::join_all;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::{Mutex, Arc};
use diesel::prelude::*;
use futures::StreamExt;
use serde_json;
use crate::commoncrawl::{CommonCrawlUrls, CommonCrawlConfig};
use std::process::exit;

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();
}

trait ResponseHandler {
    fn handle(&self, data: Vec<u8>);
}




pub async fn start_workers(app_config: AppConfig) {
    let manager =
        r2d2::ConnectionManager::<PgConnection>::new(&app_config.dbcreds);
    let pool = Arc::new(Mutex::new(r2d2::Pool::new(manager).unwrap()));
    organizer::migrate(&pool.lock().unwrap().get().unwrap());
    consumer::migrate(&pool.lock().unwrap().get().unwrap());
    let wayback_config = WaybackConfig::from(app_config.clone());
    let commoncrawl_config = CommonCrawlConfig::from(app_config.clone());
    if app_config.do_amass {
        if app_config.root_domain.is_none() {
            warn!("A Root Domain Is Required For Amass Scanning Specify One With The -r Flag");
            println!("[-] A Root Domain Is Required For Amass Scanning Specify One With The -r Flag");
            debug!("Exiting...");
            exit(-1);
        }
        println!("[+] Starting Amass This Could Take A While");
        let amass_worker = amass::populate_subs(
            app_config.root_domain.unwrap_or("".to_string()),
            app_config.amass_asn.clone(),
            app_config.amass_cidr.clone(),
            app_config.amass_config.clone(),
            &pool.lock().unwrap().get().unwrap()
        ).await;
    }
    let work = tokio::spawn(async move {
        let wayback_worker =
            WayBackUrls::new(Arc::clone(&pool).lock().unwrap().clone(), wayback_config);
        println!("[+] Starting Wayback Work");
        let wayback = tokio::spawn(async move {let result = wayback_worker.start().await;});
        let commoncrawl_worker = CommonCrawlUrls::new(Arc::clone(&pool).lock().unwrap().clone(), commoncrawl_config);
        println!("[+] Starting Commoncrawl Work");
        let commoncrawl = tokio::spawn(async move {let result = commoncrawl_worker.start().await;});
        println!("[+] Waiting For Wayback Work To Finish");
        wayback.await;
        println!("[+] Waiting For Commoncrawl Work To Finish");
        commoncrawl.await;
    });

    work.await;
    println!("[+] Finished All Work. May The Web Force Be With You")
}



#[derive(Clone, Debug)]
pub struct AppConfig {
    pub dbcreds: String,
    pub root_domain: Option<String>,
    pub async_conns: i32,
    pub do_amass: bool,
    pub amass_config: Option<String>,
    pub amass_cidr: Option<String>,
    pub amass_asn: Option<String>
}













#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
