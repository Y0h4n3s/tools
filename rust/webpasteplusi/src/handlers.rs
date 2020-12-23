
use diesel::PgConnection;
use std::collections::HashMap;
use futures::{Future, future, Stream};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use super::models::request_models::*;
use super::AppState;

pub mod hostname_types {
    use super::*;

    pub async fn hostname_protocol(data: web::Data<AppState>, payload: web::Json<HostnameProtocol>) -> String {
        debug!("{:?}", payload.data);
        "".to_string()
    }

    pub async fn hostname_much_data(data: web::Data<AppState>, payload: web::Json<HostnameMuchData>) -> String {
        debug!("{:?}", payload.data);
        "".to_string()
    }

    pub async fn hostname_own_links(data: web::Data<AppState>, payload: web::Json<HostnameMuchData>) -> String {
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
