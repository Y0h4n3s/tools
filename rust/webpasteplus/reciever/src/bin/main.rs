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
             launch_server(ServerOptions::get_from(ops));
        }
        if let Arg::Interactive(is_me) = arg {
            if !is_me.unwrap() {
                return
            }
        }
     }
    
}

fn launch_server( ops: ServerOptions) -> bool {
    //let pool: ThreadPool = ThreadPool::new(threads);
    let server = Server::new(ops);
    Server::start(&server);
    true
}
