#![allow(unused)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;
use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;

use clap::App;
use dotenv::dotenv;
use env_logger::{Builder, Target};

use consumer::handlers::AppState;

#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
    dbcreds: String
}


// this shit fixes everything up
#[actix_web::main]
 async fn main() {
     dotenv().ok();
     log::logger();
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
            debug!("Couldn't read config file: {}", err);
        })
        .ok();

    match matches.subcommand() {
        ("consume", consume_matches) => {
            let consume_matches = consume_matches.unwrap();
            let addr = matches.value_of("address")
                .map(|s| s.to_owned())
                .or(env::var("SERVER_ADDRESS").ok())
                .and_then(|addr| addr.parse().ok())
                .or(config.map(|config| config.address))
                .or_else(|| Some(([127, 0, 0, 1], 8889).into()))
                .unwrap();


            let dbcreds = consume_matches.value_of("dbcreds")
                .map(|s| s.to_owned())
                .or(dotenv::var("DATABASE_URL").ok())
                .and_then(|dbcreds| dbcreds.parse().ok())
                .or_else(||Some("".to_string())).unwrap();

            let mut rootdomain = None;
            if consume_matches.value_of("rootdomain").is_some() {
                rootdomain = Option::from(consume_matches.value_of("rootdomain").unwrap().to_string());
                debug!("Root Domain Selected: {}", &rootdomain.clone().unwrap());
            }
            debug!("Database Url: {}", dbcreds);
            let app_config = AppState {
                address: addr.to_string(),
                dbcreds: dbcreds.clone(),
                root_domain: rootdomain,
                no_file: consume_matches.is_present("no-file")
            };
            consumer::start_consuming(app_config).await;

        },
        ("organize", organize_commands) => {
            let organize_commands = organize_commands.unwrap();
            let dbcreds = organize_commands.value_of("dbcreds")
                .map(|s| s.to_owned())
                .or(dotenv::var("DATABASE_URL").ok())
                .and_then(|dbcreds| dbcreds.parse().ok())
                .or_else(||Some("".to_string())).unwrap();

            debug!("Database Url: {}", dbcreds);

            let app_config = organizer::AppState {
                db_creds: dbcreds,
                file_path: organize_commands.value_of("filepath").unwrap_or("").to_string(),
                no_file: organize_commands.is_present("no-file")
            };

            organizer::organize(app_config);

        },

        ("recon", recon_commands) => {
            let recon_commands = recon_commands.unwrap();
            debug!("Starting Recon Daemon");
            let dbcreds = recon_commands.value_of("dbcreds")
                .map(|s| s.to_owned())
                .or(dotenv::var("DATABASE_URL").ok())
                .and_then(|dbcreds| dbcreds.parse().ok())
                .or_else(||Some("".to_string())).unwrap();

            let mut rootdomain = None;
            if recon_commands.value_of("rootdomain").is_some() {
                rootdomain = Option::from(recon_commands.value_of("rootdomain").unwrap().to_string());
                debug!("Root Domain Selected: {}", &rootdomain.clone().unwrap());
            }
            let app_config = recon::AppConfig {
                dbcreds: dbcreds,
                root_domain: rootdomain
            };
            recon::start_workers(app_config).await;
        }

        _ => {
            info!("Choose One Subcommand To Continue {}", matches.usage());
        },
        _ => unimplemented!()
    }


}


