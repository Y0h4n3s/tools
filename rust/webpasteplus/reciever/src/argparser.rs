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
                let address: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-b:--bind-address"));
                let level: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-l:--parse-level"));
                let file_path: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-f:--save-location"));
                let db_creds: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-d:--db-login"));
                let no_file: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-nf:--don't-save-to-file"));
                let root_domain: Option<String> =
                    Arguments::get_arg_value(provided,
                                             String::from("-r:--root-domain"));
                //println!("root: {:?}", root_domain);

                let mut server_ops: ServerOptions = ServerOptions::new();
                server_ops
                    .set_address(address)
                    .set_level(level)
                    .set_save_path(file_path)
                    .set_db_url(db_creds)
                    .set_no_file(no_file)
                    .set_root_domain(root_domain);
                arguments.push(Arg::Server(Some(true), server_ops));
                arguments.push(Arg::Interactive(None));
            }
            "interactive" => println!("interactive"),
            _ => println!("no valeu"),
        }
        arguments
    }

    fn get_arg_value(provided: &Vec<String>, arg_keys: String)
        -> Option<String> {
        let mut result = None;
        let keys = arg_keys.split(":");
        let just_flags = vec!["-nf", "--don't-save-to-file"];
        for key in keys {
            //println!("Key {}", key);
            for arg in provided {
                //println!("{} is in provided", arg);
                if key == arg {
                    if just_flags.contains(&key) {
                        //println!("justflags contains: {}", key);
                        let value = "true".to_string().to_owned();
                        result = Option::from(value.to_owned());
                        return result.to_owned();
                    }
                    //println!("key {} equals {} arg", key, arg);
                    let index = provided.iter().position(|x| x == arg);
                    //println!("index is {}", index.unwrap());
                    let value = provided[index.unwrap() + 1].to_owned();
                    result = Option::from(value);
                    return result;
                }
            }
        }
        result.to_owned()
    }
}

pub enum Arg {
    Server(Option<bool>, ServerOptions),
    Interactive(Option<bool>),
}

pub struct ServerOptions {
    address: String,
    parse_level: usize,
    save_location: Option<String>,
    db_url: Option<String>,
    no_file: bool,
    root_domain: Option<String>,
}

impl ServerOptions {
    pub fn new() -> ServerOptions {
        let address = String::from("127.0.0.1:12345");
        let parse_level = 2;
        let save_location = Option::from(String::from("./"));
        let db_url = None;
        let no_file = false;
        let root_domain = None;

        ServerOptions {
            address,
            parse_level,
            save_location,
            db_url: db_url,
            no_file: no_file,
            root_domain: root_domain,
        }
    }

    pub fn get_from(ops: &ServerOptions) -> ServerOptions {
        let address = ops.address.clone();
        let parse_level = ops.parse_level.clone();
        let save_location = ops.save_location.clone();
        let db_creds = ops.db_url.clone();
        let no_file = ops.no_file.clone();
        let root_domain = ops.root_domain.clone();
        ServerOptions {
            address,
            parse_level,
            save_location,
            db_url: db_creds,
            no_file: no_file,
            root_domain: root_domain
        }
    }

    fn set_address(&mut self, new_address: Option<String>) -> &mut ServerOptions {
        if !new_address.is_none() {
            self.address = new_address.unwrap().to_string();
        }
        self
    }

    fn set_level(&mut self, new_level: Option<String>) -> &mut ServerOptions {
        if new_level.is_some() {
            self.parse_level = new_level
                .unwrap()
                .parse::<usize>()
                .expect("Parse Level Must Be A number Between [1-5]");
        }
        self
    }

    fn set_save_path(&mut self, new_path: Option<String>) -> &mut ServerOptions {
        if new_path.is_some() {
            self.save_location = Option::from(new_path.unwrap().to_owned());
        }
        self
    }

    fn set_db_url(&mut self, url: Option<String>) -> &mut ServerOptions {
        if url.is_some() {
            self.db_url = Option::from(url.unwrap().to_owned());
        }
        self
    }
    pub fn set_no_file(&mut self, no_file: Option<String>) -> &mut ServerOptions {
        if no_file.is_some() {
            if no_file.unwrap().eq(&"true".to_string()) {
                self.no_file = true;
            } else { self.no_file = false; }
        }
        self
    }
    pub fn set_root_domain(&mut self, root_domain: Option<String>) -> &mut ServerOptions {
        if root_domain.is_some() {
            self.root_domain = root_domain.to_owned();
        }
        self
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn parse_level(&self) -> usize {
        self.parse_level
    }


    pub fn save_location(&self) -> &Option<String> {
        &self.save_location
    }
    pub fn db_url(&self) -> &Option<String> {
        &self.db_url
    }

    pub fn no_file(&self) -> bool {
        self.no_file
    }
    pub fn root_domain(&self) -> &Option<String> {
        &self.root_domain
    }
}
