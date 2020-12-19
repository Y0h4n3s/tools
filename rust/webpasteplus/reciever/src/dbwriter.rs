#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&db_url).unwrap();
}

#[derive(Queryable)]
pub struct RootDomain {
    pub id: i32,
    pub hostname:String,
    pub ip: String,
    pub vhost: bool,
    pub notes: String,
    pub pid: i32
}