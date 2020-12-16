
use std::collections::HashMap;
pub struct Parser {
    href: String,
    level: usize,
    do_domain: bool,
}

impl Parser {
    pub fn new(href: String, level: usize, do_domain: bool) -> Parser {
        Parser {
            href,
            level,
            do_domain,
        }
    }
    pub fn parse_request(&self) -> HashMap<String,Vec<String>> {
        println!("{} {} {}", self.href, self.level, self.do_domain);
        let mut vals: HashMap<String,Vec<String>> = HashMap::new();
        let domain: Vec<String>  = vec![String::from(&self.href)];
        vals.insert(String::from("test,"), domain);
        vals
    }
}