extern crate lazy_static;
extern crate regex;
use serde_json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub struct Parser {
    data: String,
    level: usize,
    do_domain: bool,
}

impl Parser {
    pub fn new(data: String, level: usize, do_domain: bool) -> Parser {
        Parser {
            data,
            level,
            do_domain,
        }
    }
    pub fn parse_request(&self) -> Option<HashMap<String, Vec<String>>> {
        let data: &str = &self.data;
        let j: &Vec<&str> = &data.rsplit("\n").collect();
        if j[j.len() -1].find("POST") == None {
            return None
        }
        let response_json: Data = serde_json::from_str(j[0].trim_end_matches("\u{0}")).unwrap();
        let mut hrefs: String = String::default();
        for href in response_json.lines.iter() {
            &hrefs.push_str(href);
            &hrefs.push(' ');
        }
        let hosnames = get_root_hostnames(&hrefs, ".com".to_string());
        println!("Response Json: {:#?}", hrefs);
        
        let mut vals: HashMap<String, Vec<String>> = HashMap::new();
        let mut words: Vec<String> = vec![String::from(&self.data)];
        let mut filename: Vec<String> = vec!["test.txt".to_ascii_lowercase()];
        vals.insert("data".to_string(), words);
        vals.insert("filename".to_string(), filename);
        Option::from(vals)
    }
}

fn get_hostnames(hrefs: &String) -> Vec<&String>{
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    for cap in matcher.captures_iter(hrefs) {
        println!("{:?}", cap.get(1).unwrap().as_str());
    }
    vec![]
}
//TODO fix this function
fn get_root_hostsnames(hrefs: &String, root_domain: String) {
    let hostsnames: Vec<&String> = get_hostnames(hrefs);
    let re = ".?(" + &root_domain + ").?";
    let matcher = regex::Regex::new(re).unwrap();
    for hostname in hostsnames {
        if matcher.is_match(hostname) {
            println!(hostname)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    token: String,
    lines: Vec<String>,
}