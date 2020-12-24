
use diesel::{PgConnection, r2d2};
use std::collections::HashMap;
use futures::{Future, future, Stream};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::request_models::*;
use crate::actors::db_actors::*;
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub async fn index() -> String {
    "Hello world".to_string()
}


pub mod hostname_types {
    use super::*;
    use std::ops::Deref;

    pub async fn hostname_own_links(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<HostnameOwnLinks>)
        -> String {
        let conn = pool.get().map_err(|e| {
            warn!("Error Getting A Database Connection: {:?}", e);
        }).unwrap();
        if insert_hostname_own_links(payload.deref(), &conn) {
            debug!("Insert Successful");
            return "Inserted Successfully".to_string();
    }
    "Failed To Insert".to_string()
    }

    pub async fn hostname_much_data(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<HostnameMuchData>)
        -> String {
        let conn = pool.get().map_err(|e| {
            warn!("Error Getting A Database Connection: {:?}", e);
        }).unwrap();
        if insert_hostname_much_data(payload.deref(), &conn) {
            debug!("Insert Successful");
            return "Inserted Successfully".to_string();
        }
        "Failed To Insert".to_string()
    }

    pub async fn hostname_protocol(data: web::Data<AppState>, payload: web::Json<HostnameProtocol>) -> String {
        debug!("{:?}", payload.data);
        "".to_string()
    }
    pub async fn hostname_ip(data: web::Data<AppState>) -> String {
        unimplemented!();
    }
    pub async fn hostname_hrefs(data: web::Data<AppState>) -> String   {
        unimplemented!()
    }
}


#[derive(Clone, Debug)]
pub struct AppState {
    pub address: String,
    pub dbcreds: String,
    pub no_file: bool,

}