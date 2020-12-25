#![allow(unused)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel_migrations;
extern crate pretty_env_logger;
mod models;
mod helpers;
mod schema;
use dotenv::dotenv;
use crate::helpers::{DbUtils, organize_much_data};
use consumer::models::EndPoints;
use diesel::{ConnectionError, PgConnection};
use std::process::exit;

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();

}


pub fn organize(app_config: AppState) {


    let mut conn = DbUtils::new();
    match conn.check_connect(&app_config.db_creds) {
        Ok(_) => {
            let conn = conn.connect(&app_config.db_creds);
            embed_migrations!();
            embedded_migrations::run(conn.get_connection());
            debug!("Organizing Much Data Endpoint");
            organize_much_data(conn);
        }
        Err(_) => {
            error!("Error Getting A Database Connection");
            error!("Exiting...");
            exit(1);
        }
    }
}


pub struct AppState {
    pub db_creds: String,
    pub file_path: String,
    pub no_file: bool,
}
















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
