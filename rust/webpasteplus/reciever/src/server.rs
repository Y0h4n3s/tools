extern crate json;

use std::net::{TcpListener, Shutdown};
use std::io::{BufReader, BufRead, Read};
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
    const BUFFER_SIZE: usize = 65536;
    //TODO use traits to simplify data transfer
    //TODO figure out why data isn't read for bigger sizes
    let listener = TcpListener::bind(&self.address).unwrap();
    for stream in listener.incoming() {
      &self.pool.execute(move ||{
        let mut stream = stream.unwrap();
        let mut received_data: Vec<u8> = vec![];
        let peer = &mut [0u8; BUFFER_SIZE];
        let mut reader = BufReader::new(stream);
        loop {
          let mut read = reader.read(peer).unwrap();
          received_data.extend_from_slice(&peer[..read]);
          println!("Read {}", read);
          if read < BUFFER_SIZE {
            println!("No buffer");
            break
          }
        }
        println!("Recieved: {}", String::from_utf8_lossy(&*received_data));

        let parsed_data = Parser::new(String::from_utf8(received_data.clone()).unwrap().to_string(), 3, true).parse_request();
        if parsed_data.is_some() {
          paster::paste_to_file(parsed_data.unwrap());
        }
        let twoohfour = fs::read_to_string("src/responses/204.txt").unwrap();
        println!("Sending: {}", twoohfour);
       stream = reader.into_inner();
        stream.write(twoohfour.as_bytes()).unwrap();
        stream.flush();
        stream.shutdown(Shutdown::Both).unwrap();
        received_data.clear();
        return;
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