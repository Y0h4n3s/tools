#![allow(unused)]

mod handlers;
mod actors;
mod models;
mod schema;
mod helpers;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::fs::File;
use std::io::Read;

use std::{env, io};
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;
use dotenv::dotenv;
use clap::App as CApp;
use serde_json;
use serde_derive::Deserialize;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use self::handlers::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use actix_web::http::{Method};
use std::process::exit;


#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
    dbcreds: String
}

#[derive(Clone, Debug)]
pub struct AppState {
    address: String,
    dbcreds: String,
    no_file: bool,

}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

 fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = CApp::from_yaml(yaml).get_matches();

    let config = File::open("service.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            serde_json::from_str::<Config>(&buffer)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok();




    match matches.subcommand_name() {
        Some("consume") => {
            let addr = matches.value_of("address")
                .map(|s| s.to_owned())
                .or(env::var("SERVER_ADDRESS").ok())
                .and_then(|addr| addr.parse().ok())
                .or(config.map(|config| config.address))
                .or_else(|| Some(([127, 0, 0, 1], 8889).into()))
                .unwrap();


            let dbcreds = matches.value_of("dbcreds")
                .map(|s| s.to_owned())
                .or(dotenv::var("DATABASE_URL").ok())
                .and_then(|dbcreds| dbcreds.parse().ok())
                .or_else(||Some("".to_string()));

            let app_config = AppState {
                address: addr.to_string(),
                dbcreds: dbcreds.unwrap(),
                no_file: matches.is_present("no-file")
            };
            start_consuming(app_config);


        }
        None => info!("Choose One Subcommand To Continue"),
        _ => unimplemented!()
    }

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.



}


#[actix_web::main]
async fn start_consuming(app_config: AppState) -> std::io::Result<()> {

    let manager = r2d2::ConnectionManager::<PgConnection>::new(&app_config.dbcreds);
    let pool = r2d2::Pool::builder().build(manager).map_err(|e| {
        warn!("Failed To Connect To The Database: {:?}",e);
        if app_config.no_file {
            debug!("Exiting...");
            exit(1);
        }
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