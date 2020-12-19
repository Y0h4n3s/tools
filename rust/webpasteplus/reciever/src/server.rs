extern crate json;

use std::net::{TcpListener, Shutdown};
use std::io::{BufReader, BufRead, Read};
use std::io::Write;
use std::fs;
use actix_web::{get, post,web, App, HttpResponse, HttpServer, Responder};
use crate::ThreadPool;
use crate::separator::Parser;
use crate::paster;
use actix_web::http::{StatusCode, Method};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use crate::argparser::ServerOptions;

pub struct Server {
  ops: ServerOptions,

}


impl Server {
  pub fn new(ops: ServerOptions) -> Server {
    Server { ops }
  }

pub fn start(&self) -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
        .route("/", web::method(Method::OPTIONS).to(Server::options))
        .route("/", web::post().to(Server::data_post))
  })
      .bind(self.ops.get_address())?
      .run()
}

  fn data_post(info: Json<Data>) -> impl Responder {
    println!("{:?}", info);
    let parser = Parser::new(info.into_inner().everything_else, 3, true);
    let parsed_data = parser.parse_request();
    let pasted_to_file = paster::paste_to_file(parsed_data.unwrap());
    HttpResponse::build(StatusCode::OK)
        .with_header("Access-Control-Allow-Origin", "chrome-extension://ojddniephhbohamkfcejdoomfdcfbjig")
        .with_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .with_header("Access-Control-Allow-Headers", "Content-Type")
  }
  fn options() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .with_header("Access-Control-Allow-Origin", "chrome-extension://ojddniephhbohamkfcejdoomfdcfbjig")
        .with_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .with_header("Access-Control-Allow-Headers", "Content-Type")
  }

}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
  token: String,
  everything_else: Vec<HashMap<String,Vec<String>>>,
}