use std::env::*;

pub struct Arguments {
    arguments: Vec<Arg>,
}

impl Arguments {
    pub fn new() -> Arguments {
        let provided: Vec<String> = args().collect();
        let arguments: Vec<Arg> = Arguments::map_args(&provided);

        Arguments { arguments }
    }

    pub fn get_args(&self) -> &Vec<Arg> {
        &self.arguments
    }

    fn map_args(provided: &Vec<String>) -> Vec<Arg> {
        let root: &str = &provided[1];
        let mut arguments: Vec<Arg> = Vec::new();
        match root {
            "server" => {
                let address: Option<&String> =
                    Arguments::get_arg_value(provided, String::from("-b:--bind-address"));
                let level: Option<&String> =
                    Arguments::get_arg_value(provided, String::from("-l:--parse-level"));
                
                println!(
                    "address found: {}, level found: {}",
                    address.unwrap(),
                    level.unwrap()
                );

                let mut server_ops: ServerOptions = ServerOptions::new();
                server_ops
                    .set_address(address)
                    .set_level(Option::from(&level.unwrap().parse::<usize>().unwrap()));
                arguments.push(Arg::Server(Some(true), server_ops));
                arguments.push(Arg::Interactive(None));
            }
            "interactive" => println!("interactive"),
            _ => println!("no valeu"),
        }
        arguments
    }

    fn get_arg_value(provided: &Vec<String>, arg_keys: String) -> Option<&String> {
        let mut result = None;
        let keys = arg_keys.split(":");
        for key in keys {
            println!("Key {}", key);
            for arg in provided {
                if key == arg {
                    println!("key {} equals {} arg", key, arg);
                    let index = provided.iter().position(|x| x == arg);
                    println!("index is {}", index.unwrap());
                    let value = &provided[index.unwrap() + 1];
                    result = Option::from(value);
                    return result;
                }
            }
        }
        result
    }
}

pub enum Arg {
    Server(Option<bool>, ServerOptions),
    Interactive(Option<bool>),
}

pub struct ServerOptions {
    address: String,
    parse_level: usize,
    threads: usize,
}

impl ServerOptions {
    fn new() -> ServerOptions {
        let address = String::from("127.0.0.1:12345");
        let parse_level = 2;
        let threads = 10;
        ServerOptions {
            address,
            parse_level,
            threads,
        }
    }

    fn set_address(&mut self, new_address: Option<&String>) -> &mut ServerOptions {
        if !new_address.is_none() {
            self.address = new_address.unwrap().to_string();
        }
        self
    }

    fn set_level(&mut self, new_level: Option<&usize>) -> &mut ServerOptions {
        if !new_level.is_none() {
            self.parse_level = *new_level.unwrap();
        }
        self
    }

    fn set_threads(&mut self, new_threads: Option<&usize>) -> &mut ServerOptions {
        if !new_threads.is_none() {
            self.threads = *new_threads.unwrap();
        }
        self
    }

    
    pub fn get_address(&self) -> &String {
        &self.address
    }

    pub fn get_parse_level(&self) -> usize {
        self.parse_level
    }
    pub fn get_threads(&self)-> usize {
        self.threads
    }
}
