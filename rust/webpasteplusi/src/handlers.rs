
use diesel::PgConnection;
use std::collections::HashMap;
use futures::{Future, future, Stream};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use super::models::request_models::*;
use super::AppState;
use super::DbPool;
use super::actors::db_actors::*;


pub async fn index() -> String {
    "Hello world".to_string()
}


pub mod hostname_types {
    use super::*;
    use std::ops::Deref;

    pub async fn hostname_protocol(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<HostnameProtocol>)
        -> String {
        let conn = pool.get().map_err(|e| {
            warn!("Error Getting A Database Connection: {:?}", e);
        }).unwrap();
        if insert_hostname_protocol(payload.deref(), &conn) {
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

    pub async fn hostname_own_links(data: web::Data<AppState>, payload: web::Json<HostnameOwnLinks>) -> String {
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
