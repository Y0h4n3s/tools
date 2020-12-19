extern crate lazy_static;
extern crate regex;
extern crate permutate;
use permutate::Permutator;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;
use std::hash::Hash;
use json::from;
use self::regex::Captures;


//TODO implement tag source for extractd links
pub struct Parser {
    data: Vec<HashMap<String,Vec<String>>>,
    level: usize,
    do_domain: bool,
}

impl Parser {
    pub fn new(data: Vec<HashMap<String,Vec<String>>>, level: usize, do_domain: bool) -> Parser {
        Parser {
            data,
            level,
            do_domain,
        }
    }
    pub fn parse_request(&self) -> Option<HashMap<String, HashMap<String, HashMap<String,Vec<String>>>>> {

        let mut store_data: HashMap<String, HashMap<String, HashMap<String,Vec<String>>>> = HashMap::new();
        let request_json = self.data.clone();
        let the_useful_data = request_json[1].get("full_links").unwrap().to_owned();
        let mut hrefs: String = "".to_string();
        for href in the_useful_data.iter() {
            //println!("{}", href);
            &hrefs.push_str(href);
            &hrefs.push(' ');
        }

        //TODO implement choice filtering by hostname
        let hostnames: HashMap<String,Vec<String>> = get_root_hostnames(&hrefs, ".com".to_string());
        let mut hostnames_entry: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        hostnames_entry.insert("hostnames".to_string(), hostnames);

        store_data.insert("hostnames".to_string(), hostnames_entry);

        let link_dirs = request_json[2].clone();
        let wordlists = get_wordlist_all(link_dirs);
        store_data.insert("wordlists".to_string(), wordlists);

        Option::from(store_data)
    }
}

fn get_wordlist_all(hrefs: HashMap<String,Vec<String>>) -> HashMap<String, HashMap<String, Vec<String>>> {
    let matcher = regex::Regex::new(r".?(?:https?:)?//(?:[\w\-.]+)?(/[^\s\n]+).?").unwrap();
    let mut wordlists: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for (href, dirs) in hrefs.clone() {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        hm.insert("hostname".to_string(), vec![get_hostname(&href)]);
        hm.insert("protocol".to_string(), vec![get_protocol(&href)]);
        hm.insert("exact".to_string(), dirs);
        wordlists.insert(href.to_string(), hm);
    }
    wordlists
}

fn extract_words(href: &str, level: usize) -> HashMap<String, Vec<String>> {
    let mut word_holder: HashMap<String, Vec<String>> = HashMap::new();
    word_holder.insert(href.to_owned(),dirs_only(href));
    word_holder
}

fn dirs_only(href: &str) -> Vec<String>{
    let mut wordlist: Vec<String> = Vec::new();
    let is_dir = regex::Regex::new(r"/$").unwrap().is_match(href);
    let mut paths: Vec<&str> = href.split_terminator('/').collect();
    for path in paths {
        wordlist.push(path.to_owned());
    }
    wordlist
}

fn get_protocol(href: &String) -> String {
    let matcher = regex::Regex::new(r".?^(https?)://.?").unwrap();
    let protocol = matcher.captures(href).unwrap()[1].to_string();
    protocol
}
fn get_hostname(href: &String) -> String {
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    let hostname = matcher.captures(href).unwrap()[1].to_string();
    hostname
}
fn get_hostnames(hrefs: &String) -> HashMap<String,Vec<String>> {
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    let mut hostnames: HashMap<String, Vec<String>> = HashMap::new();
    let mut hostnames_store: Vec<String> = Vec::new();
    for cap in matcher.captures_iter(hrefs) {
        //println!("{:?}", &cap[1]);
        let hostname = &cap[1];
        hostnames_store.push(hostname.to_string())
    }
    hostnames_store.sort_unstable();
    hostnames_store.dedup();
    hostnames.insert("hostnames".to_string(), hostnames_store);
    hostnames
}

fn get_root_hostnames(hrefs: &String, root_domain: String) -> HashMap<String, Vec<String>> {
    let mut hostnames: Vec<String> = get_hostnames(hrefs).get("hostnames").unwrap().clone();
    let re = format!(r".?({}).?", &root_domain);
    let matcher = regex::Regex::new(&re).unwrap();
    let mut valids: HashMap<String, Vec<String>> = HashMap::new();
    let mut valid_hostnames: Vec<String> = Vec::new();
    for hostname in hostnames.clone() {
        //println!("{}",hostname);
        if matcher.is_match(&hostname) {
            if !valid_hostnames.contains(&hostname) {
                valid_hostnames.push(hostname);
            }
        }
    }
    valids.insert("valid-hostnames".to_string(), valid_hostnames);
    valids.insert("hostnames".to_string(), hostnames);
    valids
}

