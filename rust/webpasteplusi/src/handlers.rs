use gotham::state::{State, FromState};
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{StatusCode, Response};
use diesel::PgConnection;
use hyper::Body;
use std::collections::HashMap;
use crate::Repo;
use futures::{Future, future, Stream};
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use std::str::from_utf8;

pub fn index(state: State) -> (State, &'static str) {
(state, "HELLO_WORLD")
}

pub fn cors_options(state: State) -> (State, Response<Body>) {
    let mut res = create_empty_response(&state, StatusCode::OK);

    let headers = res.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "POST, OPTIONS, GET".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());

    (state, res)
}

pub mod hostname_types {

    use super::*;
    pub fn hostname_protocol(mut state: State) -> (State, Response<Body>) {
        let repo = Repo::borrow_from(&state).clone();
        let res = create_empty_response(&state, StatusCode::OK);
        let st = extract_json(&mut state);
        println!("it workde");
        (state, res)

    }
    pub fn hostname_ip(state: State) -> (State, Response<Body>) {
        unimplemented!();
    }
    pub fn hostname_hrefs(state: State) -> (State, Response<Body>)   {
        unimplemented!()
    }
}

pub fn extract_json(state: &mut State) -> String
{
    async {
        let b = Body::take_from(state)
            .concat2();
        println!("{:?}", b);
        println!("didnt work");

    };
    "".to_string()
}

#[derive(Deserialize)]
pub struct Test {
    test: String
}