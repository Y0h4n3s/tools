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
use diesel::*;
use diesel::{ConnectionError, PgConnection, QueryDsl, ExpressionMethods};
use dotenv::dotenv;
use std::process::Command;
use consumer::models::EndPoints;

use crate::helpers::{DbUtils, organize_dom_xss, organize_much_data, organize_own_links, update_last_processed};
use diesel::r2d2::{PooledConnection, ConnectionManager};

pub mod models;
pub mod helpers;
pub mod schema;

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();
}


pub fn migrate(conn: &PooledConnection<ConnectionManager<PgConnection>>) {
    embed_migrations!();
    embedded_migrations::run(conn);
}
pub fn organize(app_config: AppState) {
    let mut conn = DbUtils::new();

    conn.connect(&app_config.db_creds);
    migrate(&conn.get_connection());
    debug!("Organizing...");
    println!("[+] Organizing...");
    use crate::schema::configs::dsl::*;
    let last_stop = configs
        .filter(key.eq("last_processed_dump_id"))
        .select(value)
        .get_result::<Option<String>>(&conn.get_connection())
        .unwrap_or(Option::from("1".to_string()))
        .unwrap()
        .parse::<i32>()
        .unwrap_or(1);
    debug!("Last sotpped at: {:?}", last_stop);
    println!("[+] Last sotpped at: {:?}", last_stop);

    organize_dom_xss(&conn.get_connection(), last_stop);

    organize_much_data(&conn, last_stop);
    organize_own_links(&conn, last_stop);

    update_last_processed(&conn.get_connection());

}

pub fn pack(dbcreds: String) {
    let mut packer_cmd = Command::new("pg_dump");
    packer_cmd
        .arg("-f")
        .arg("backup.sql")
        .arg(&dbcreds);
    debug!("Packer Cmd: {:?}", &packer_cmd);
    let out = packer_cmd.output();
    match out {
        Ok(result) => {
            println!("[+] Backup Complete.");
            debug!("{:?}", result)
        }
        Err(e) => println!("[-] An Error Occured While Attempting To Backup: {:?}", e)
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
