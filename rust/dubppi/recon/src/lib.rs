#![allow(unused)]
#[macro_use] extern crate log;
extern crate pretty_env_logger;
mod thread_pool;
mod models;
mod wayback;

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
pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();
}

trait ResponseHandler {
    fn handle(&self, data: Vec<u8>);
}




pub async fn start_workers(app_config: AppConfig) {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(&app_config.dbcreds);
    let pool = Arc::new(Mutex::new(r2d2::Pool::new(manager).unwrap()));
    let wayback_config = WaybackConfig::from(app_config);
    let work = tokio::spawn(async move {
        let wayback_worker = WayBackUrls::new(Arc::clone(&pool).lock().unwrap().clone(), wayback_config);
        let wayback = tokio::spawn(async move {let result = wayback_worker.start().await;});
        wayback.await;
    }).await;
    debug!("{:?}", work);
}



#[derive(Clone, Debug)]
pub struct AppConfig {
    pub dbcreds: String
}













#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
