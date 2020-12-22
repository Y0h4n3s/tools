mod handlers;
mod schema;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::fs::File;
use std::io::Read;

use std::{env, io};
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;
use dotenv::dotenv;
use clap::App;
use serde_json;
use serde_derive::Deserialize;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use gotham::router::Router;
use gotham::router::builder::*;

use self::handlers::*;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::new_pipeline;
use gotham_middleware_diesel::DieselMiddleware;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;



#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
    dbcreds: String
}

 fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

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

            start_consuming(addr, dbcreds.clone());


        }
        None => info!("Choose One Subcommand To Continue"),
        _ => unimplemented!()
    }

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.



}

fn router(repo: Repo) -> Router {
    let (chain, pipeline) =
        single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());


    build_router(chain, pipeline, |route| {
        route.get_or_head("/").to(index);
        route.options("/").to(cors_options);
        route.options("*").to(cors_options);


        route.scope("/hostname", | route | {
            route.options("/hostname_hrefs").to(cors_options);
            route.options("/hostname_protocol").to(cors_options);
            route.options("/hostname_ip").to(cors_options);
            route.post("/hostname_protocol").to(hostname_types::hostname_protocol);
            route.post("/hostname_hrefs").to(hostname_types::hostname_protocol);
            route.post("/hostname_ip").to(hostname_types::hostname_protocol);
        })
    })
}
fn start_consuming(addr: SocketAddr, dbcreds: Option<String>) {

    // Run this server for... forever!
    debug!("Listening For Requests on {:?} ...", addr);
    gotham::start(addr.to_string(), router(Repo::new(&dbcreds.unwrap())));
}