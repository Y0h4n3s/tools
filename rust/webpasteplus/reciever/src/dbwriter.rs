use std::collections::HashMap;
use std::env;

use diesel::associations::HasTable;
use diesel::debug_query;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::result::Error;
use juniper::futures::AsyncReadExt;

use crate::dbmodels;
use crate::schema::*;

//TODO improve with traits
pub struct DbWriter {
    db_url: String,
}

impl DbWriter {
    pub fn new() -> DbWriter {
        let db_url = "".to_string();
        let db_connection: Option<PgConnection> = None;
        DbWriter { db_url }
    }
    pub fn connect(&mut self, db_url: String) -> Pool<ConnectionManager<PgConnection>> {
        let manager =
            Option::from(ConnectionManager::<PgConnection>::new(
                "postgres://postgres:postgres-password@localhost/datasets"
            ));
        let mut pool = Option::from(r2d2::Pool::builder()
            .build(manager.unwrap())
            .expect("Failed to create pool."));
        pool.expect("Something Went Wrong With The Database Connection")
    }

    pub fn insert_raw<'a>(
        data: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
        conn: &PgConnection
    )
        -> Result<dbmodels::RootDomain, diesel::result::Error> {
        use crate::schema::root_domains::dsl::root_domains;
        //println!("db writer is in the house");
        let host: String = "blablatest.com".to_string();
        let pi: i32 = 1;
        let root_domain = dbmodels::RootDomain {
            id: 0,
            hostname: host,
        };
        //println!("db writer was successful?");
        let query = diesel::insert_into(root_domains)
            .values(&root_domain);
        println!("db writer is trying to insert data {} ", debug_query::<Pg, _>(&query));
        //query.execute(conn)?;

        Ok(root_domain)
    }
}