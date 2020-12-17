use reciever::ThreadPool;
use reciever::argparser::*;
use reciever::server::*;

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
             launch_server(address, level, *threads);
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
    let server = Server::new(address, pool);
    server.start();
    true
}
