#![allow(unused)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::process::exit;
use std::thread;

use diesel::{ConnectionError, PgConnection};
use dotenv::dotenv;

use consumer::models::EndPoints;

use crate::helpers::{DbUtils, organize_dom_xss, organize_much_data, organize_own_links, update_last_processed};

mod models;
pub mod helpers;
pub mod schema;

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();
}


pub fn organize(app_config: AppState) {
    let mut conn = DbUtils::new();

    conn.connect(&app_config.db_creds);
    embed_migrations!();
    embedded_migrations::run(&conn.get_connection());
    debug!("Organizing...");
    organize_dom_xss(&conn.get_connection());

    organize_much_data(&conn);
    organize_own_links(&conn);

    update_last_processed(&conn.get_connection());

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
