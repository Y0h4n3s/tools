
use diesel::{PgConnection, r2d2};
use std::collections::HashMap;
use futures::{Future, future, Stream};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::request_models::*;
use crate::actors::db_actors::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub async fn index() -> String {
    "Hello world".to_string()
}


pub mod dom_types {
    use super::*;
    use std::ops::Deref;
    use diesel::r2d2::PooledConnection;

    pub async fn dom_xss_sinks(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<DomXssSinks>
    ) -> String {
        let conn = get_conn(&pool);
        debug!("Payload: {:?}", payload.deref());
        if insert_dom_xss_sinks(payload.deref(), &conn,) {
            debug!("Insert Successful");
            return "Inserted Successfully".to_string();
        }
        "Failed To Insert".to_string()
    }

    pub async fn dom_xss_sources(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<DomXssSources>
    ) -> String {
        let conn = get_conn(&pool);
        if insert_dom_xss_sources(payload.deref(), &conn,) {
            debug!("Insert Successful");
            return "Inserted Successfully".to_string();
        }
        "Failed To Insert".to_string()
    }

    pub async fn dom_own_links(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<DomOwnLinks>)
        -> String {
        let conn = get_conn(&pool);
        if insert_hostname_own_links(payload.deref(), &conn, data.root_domain.clone()) {
            debug!("Insert Successful");
            return "Inserted Successfully".to_string();
    }
    "Failed To Insert".to_string()
    }

    pub async fn dom_much_data(
        data: web::Data<AppState>,
        pool: web::Data<DbPool>,
        payload: web::Json<DomMuchData>)
        -> String {
        let conn= get_conn(&pool);
        if insert_hostname_much_data(payload.deref(), &conn, data.root_domain.clone()) {
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


    fn get_conn(pool: &web::Data<DbPool>) -> PooledConnection<ConnectionManager<PgConnection>> {
        let conn = pool.get().map_err(|e| {
            warn!("Error Getting A Database Connection: {:?}", e);
        }).unwrap();
        conn
    }

}


#[derive(Clone, Debug)]
pub struct AppState {
    pub address: String,
    pub dbcreds: String,
    pub root_domain: Option<String>,
    pub no_file: bool,

}
