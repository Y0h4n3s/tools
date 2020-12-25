#![allow(unused)]

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::{env, io};
use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::process::exit;

use actix_web::{App, get, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::http::Method;
use clap::App as CApp;
use diesel::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use serde_derive::Deserialize;
use serde_json;

pub mod handlers;
mod actors;
pub mod models;
pub mod schema;
mod helpers;
use dotenv::dotenv;
use crate::handlers::{hostname_types, index, AppState};

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[actix_web::main]
pub async fn start_consuming(app_config: AppState) -> std::io::Result<()> {

    let manager = r2d2::ConnectionManager::<PgConnection>::new(&app_config.dbcreds);
    let pool = r2d2::Pool::builder().build(manager).map_err(|e| {
        warn!("Failed To Connect To The Database: {:?}",e);
            debug!("Exiting...");
            exit(1);

    }).unwrap();

    // Run this server for... forever!
    debug!("Listening For Requests on {:?} ...", &app_config.address);
    let address = app_config.address.clone();
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().limit(1005535);
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .data(app_config.clone())
            .service(
                web::scope("/hostname")
                    .app_data(json_config)
                    .route("/hostname_protocol", web::post().to(hostname_types::hostname_protocol))
                    .route("/hostname_hrefs", web::post().to(hostname_types::hostname_hrefs))
                    .route("/hostname_ip", web::post().to(hostname_types::hostname_ip))
                    .route("/hostname_much_data", web::post().to(hostname_types::hostname_much_data))
                    .route("/hostname_own_links", web::post().to(hostname_types::hostname_own_links))
            )
            .route("/", web::get().to(index))



    })
        .bind(address)?
        .run()
        .await
}