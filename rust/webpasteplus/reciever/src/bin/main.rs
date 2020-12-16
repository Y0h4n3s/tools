use reciever::paster;
use reciever::separator::Parser;
use reciever::ThreadPool;
use reciever::argparser::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
fn main() {
    //let (tx: Sender, rx: Receiver) = mpsc::channel();
    let argstoo = Arguments::new();
    for arg in argstoo.get_args().iter() {
        if let Arg::Server(is_me, ops) = arg {
            if !is_me.unwrap() {
                continue
            }
             println!("its a sever {}", is_me.unwrap());
             println!("with a value {}", ops.get_address());
             let address = ops.get_address();
             let level = ops.get_parse_level();
             let threads = &ops.get_threads();
             launch_server(&address, level, *threads);
        }
        if let Arg::Interactive(is_me) = arg {
            if !is_me.unwrap() {
                return
            }
        }
     }
    
}

fn launch_server(address: &String, _parse_level: usize, threads: usize) -> bool {
    println!("Listening on {}", address);
    let pool: ThreadPool = ThreadPool::new(threads);
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let mut buffer = [0; 1020];
        let mut stream = stream.unwrap();
        stream.read(&mut buffer).unwrap();
        pool.execute(|| {
            let parsed: HashMap<String, Vec<String>> = Parser::new(String::from("testurl"), 10, true).parse_request();
            let pasted = paster::paste_to_file(parsed);
            println!("{}", pasted);
        });
        println!("{}", String::from_utf8_lossy(&buffer));
    }
    thread::spawn(|| {
        for _i in 1..10 {
            thread::current().name();
            println!("from other");
            thread::sleep(Duration::from_secs(3));
        }
    });
    for _i in 1..14 {
        println!("{}", thread::current().name().expect("didnt work also"));
        thread::sleep(Duration::from_secs(2));
    }
    true
}
