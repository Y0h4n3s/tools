#![allow(unused)]

#[macro_use] extern crate clap;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
#[macro_use] extern crate serde_derive;

use dotenv::dotenv;
use consumer::*;
use consumer::handlers::AppState;
use std::fs::File;
use std::net::SocketAddr;
use std::{io, env};
use std::io::Read;
use clap::App;

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

            let app_config = AppState {
                address: addr.to_string(),
                dbcreds: dbcreds.unwrap(),
                no_file: matches.is_present("no-file")
            };

            consumer::start_consuming(app_config);

        },
        Some("organize") => {
            let dbcreds = matches.value_of("dbcreds")
                .map(|s| s.to_owned())
                .or(dotenv::var("DATABASE_URL").ok())
                .and_then(|dbcreds| dbcreds.parse().ok())
                .or_else(||Some("".to_string()));

            let app_config = organizer::AppState {
                db_creds: dbcreds.unwrap_or("".to_string()),
                file_path: matches.value_of("filepath").unwrap_or("").to_string(),
                no_file: matches.is_present("no-file")
            };

            organizer::organize(app_config);
        },
        None => info!("Choose One Subcommand To Continue"),
        _ => unimplemented!()
    }

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.



}


