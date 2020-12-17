extern crate json;

use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::fs;
use crate::ThreadPool;
use crate::separator::Parser;
use crate::paster;


pub struct Server {
  address: String,
  pool: ThreadPool,
}


impl Server {
  pub fn new(address: &str, pool: ThreadPool) -> Server {
    let address = String::from(address);
    
    Server {address, pool}
  }

  pub fn start(&self) -> &Server {
    let listener = TcpListener::bind(&self.address).unwrap();
    for stream in listener.incoming() {
      &self.pool.execute(move ||{
        let mut stream = stream.unwrap();
        
        let peer: &mut [u8] = &mut [0;4096];
        stream.read(peer).unwrap();
        println!("Recieved: {}", String::from_utf8_lossy(peer));
        let parsed_data = Parser::new(String::from_utf8_lossy(peer).to_string(), 3, true).parse_request();
        if parsed_data.is_some() {
          paster::paste_to_file(parsed_data.unwrap());
        }
        let twoohfour = fs::read_to_string("src/responses/204.txt").unwrap();
        println!("Sending: {}", twoohfour);
        stream.write(twoohfour.as_bytes()).unwrap();
      });
    }
    /* for stream in listener.incoming() {
        let mut buffer = [0; 1020];
        let mut stream = stream.unwrap();
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
    } */
    self
  }
}