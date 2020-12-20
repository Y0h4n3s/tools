extern crate diesel;

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::*;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::http::{StatusCode};
use actix_web::web::Json;
use diesel::{PgConnection};
use diesel::r2d2::{self, ConnectionManager, Pool};
use serde::{Deserialize, Serialize};

use reciever::{paster};
use reciever::argparser::*;
use reciever::argparser::ServerOptions;
use reciever::dbwriter;
use reciever::dbwriter::DbWriter;
use reciever::separator::Parser;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_web::main]
async fn main() -> juniper::futures::io::Result<()> {
    //let (tx: Sender, rx: Receiver) = mpsc::channel();
    let argstoo = Arguments::new();
    let args = argstoo.get_args();
    let mut ops = ServerOptions::new();
    for arg in args {
        match arg {
            Arg::Server(_is_me, ops1) => {
                ops = ServerOptions::get_from(ops1);
            }
            _ => {}
        }
    }
    let data =
        web::Data::new(Mutex::new(ServerOptions::get_from(&ops)));
    let mut db_writer = DbWriter::new();
    let mut pool: Option<Pool<ConnectionManager<PgConnection>>> = None;
    if ops.db_url().is_some() {
        pool =
            Option::from(
                db_writer
                    .connect(ops.db_url().as_ref().unwrap().to_string()));
    }

    HttpServer::new(move || {
        App::new().data(pool.clone().unwrap())
            .service(options)
            .service(data_post)
            .app_data(data.clone())
    })
        .bind(ops.address())?
        .run()
        .await
}

//handle all saving operations
#[post("/")]
async fn data_post(pool: web::Data<DbPool>,
                   info: Json<Data>,
                   data: web::Data<Mutex<ServerOptions>>,
) -> Result<HttpResponse, Error> {

    //get the user options from the data store
    let data = data.lock().unwrap();

    //get the database connection grom the event pool
    let conn =
        pool.get().expect("Couldn't Get A Database Connection From The Pool");
    println!("{:?} ", data.deref().no_file());

    //parse the request from the browser into a usable format
    let root_domain = data.root_domain();
    println!("{:?}", root_domain.clone());
    let parser =
        Parser::new(
            info.into_inner().everything_else,
            3,
            root_domain.clone().unwrap_or("".to_string()).to_owned(),
        );
    let parsed_data = parser.parse_request();

    //write to the database if a valid database connection is provided
    let db_data = parsed_data.clone();
    let file_data = parsed_data.clone();
    if data.deref().db_url().is_some() {
        web::block(move || {
            //println!("db writer is called");
            dbwriter::DbWriter::insert_raw(db_data.unwrap().clone(), &conn)
        })
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    }

    //save to files if it hasn't been disabled
    if !data.deref().no_file() {
        paster::paste_to_file(
            file_data.unwrap().clone(),
            data.deref().save_location().as_ref().unwrap(),
        );
    }

    //respond with a 200 OK
    Ok(HttpResponse::Ok()
        .header(
            "Access-Control-Allow-Origin",
            "chrome-extension://ojddniephhbohamkfcejdoomfdcfbjig"
        )
        .header(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type")
        .finish()
    )
}

//initiate a successful cors connection with the browser
#[options("/")]
async fn options() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .with_header("Access-Control-Allow-Headers", "Content-Type")
}


#[derive(Serialize, Deserialize, Debug)]
struct Data {
    token: String,
    everything_else: Vec<HashMap<String, Vec<String>>>,
}