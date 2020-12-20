extern crate regex;


use std::collections::HashMap;

use juniper::sa::_core::hash::Hash;

//TODO implement tag source for extractd links
pub struct Parser {
    data: Vec<HashMap<String, Vec<String>>>,
    level: usize,
    root_domain: String,
}

#[warn(dead_code)]
impl Parser {
    pub fn new(data: Vec<HashMap<String, Vec<String>>>, level: usize, root_domain: String)
        -> Parser {
        Parser {
            data,
            level,
            root_domain,
        }
    }
    pub fn parse_request(&self)
                         -> Option<HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>> {
        let mut store_data: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> =
            HashMap::new();
        let request_json = self.data.clone();
        let the_useful_data = request_json[1].get("full_links").unwrap().to_owned();
        //TODO implement choice filtering by hostname
        println!("{}", self.root_domain);
        let hostnames: HashMap<String, HashMap<String, Vec<String>>> =
            get_root_hostnames(&the_useful_data, self.root_domain.clone());
        let mut hostnames_entry: HashMap<String, HashMap<String, Vec<String>>> =
            one_love(hostnames.clone(), self.root_domain.clone());

        store_data.insert("hostnames".to_string(), hostnames_entry);

        let link_dirs = request_json[2].clone();
        let wordlists = get_wordlist_all(link_dirs);
        store_data.insert("wordlists".to_string(), wordlists);

        Option::from(store_data)
    }
}

fn one_love(domain_hrefs: HashMap<String, HashMap<String, Vec<String>>>, root: String)
            -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut from_root: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let starts_with_re = regex::Regex::new(&format!("^{}", root)).unwrap();
    let mut subs: HashMap<String, Vec<String>> = HashMap::new();
    subs.insert(root.clone(), Vec::new());
    for (hostname, href) in domain_hrefs.get("valids").unwrap() {
        if starts_with_re.is_match(&hostname) {
            //println!("re matches: {} {:?}",hostname, href);
            let mut other = subs[hostname].clone();
            (*subs.entry(hostname.clone()).or_insert(Vec::new())).append(&mut other);
        } else {
            println!("re doesn't matches: {} {:?}", hostname, href);
            subs.insert(hostname.clone(), href.clone());
        }
    }
    from_root.insert(root.clone(), subs.clone());
    for (hostname, href) in from_root.clone() {
        for (hostname, mut hrefs) in href.to_owned() {
            hrefs.sort_unstable();
            hrefs.dedup();
            // println!("{} : {:?}", hostname, hrefs);
        }
    }
    from_root
}

fn get_wordlist_all(hrefs: HashMap<String, Vec<String>>)
                    -> HashMap<String, HashMap<String, Vec<String>>> {
    //let matcher = regex::Regex::new(r".?(?:https?:)?//(?:[\w\-.]+)?(/[^\s\n]+).?").unwrap();
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

fn extract_words(href: &str) -> HashMap<String, Vec<String>> {
    let mut word_holder: HashMap<String, Vec<String>> = HashMap::new();
    word_holder.insert(href.to_owned(), dirs_only(href));
    word_holder
}

fn dirs_only(href: &str) -> Vec<String> {
    let mut wordlist: Vec<String> = Vec::new();
    //let is_dir = regex::Regex::new(r"/$").unwrap().is_match(href);
    let paths: Vec<&str> = href.split_terminator('/').collect();
    for path in paths {
        wordlist.push(path.to_owned());
    }
    wordlist
}

fn get_protocol(href: &String) -> String {
    let matcher = regex::Regex::new(r".?^(https?|wss|ftp|ssh|smtp)://.?").unwrap();
    let protocol = matcher.captures(href)
        .expect(&format!("Couldn't Extract Protocol From {}", href))[1].to_string();
    protocol
}

fn get_hostname(href: &String) -> String {
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    let hostname = matcher.captures(href).unwrap()[1].to_string();
    hostname
}

fn get_hostnames(hrefs: &Vec<String>)
                 -> HashMap<String, Vec<String>> {
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    let mut hostnames: HashMap<String, Vec<String>> = HashMap::new();
    for href in hrefs {
        for cap in matcher.captures_iter(href) {
            //println!("{:?}", &cap[1]);
            let mut hostname = &cap[1];
            if hostnames.contains_key(hostname) {
                //println!("already exists: {:?}", hostname);
                let mut my_hrefs = hostnames[hostname].clone();
                my_hrefs.push(href.clone());
                (*hostnames.entry(hostname.parse().unwrap())
                    .or_insert(Vec::new())).append(&mut my_hrefs);
            } else {
                let mut my_hrefs = Vec::new();
                my_hrefs.push(href.clone());
                hostnames.insert(hostname.to_string(), my_hrefs);
            }
        }
    }
    hostnames
}

fn get_root_hostnames(hrefs: &Vec<String>, root_domain: String)
                      -> HashMap<String, HashMap<String, Vec<String>>> {
    let link_hostnames = get_hostnames(hrefs).clone();
    //println!("theud: {:?}", link_hostnames);

    let re = format!(r"({}).?", &root_domain);
    println!("Regex: {} {}", re, root_domain);
    let mut everything: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let matcher = regex::Regex::new(&re).unwrap();
    let mut valids: HashMap<String, Vec<String>> = HashMap::new();
    let mut invalids: HashMap<String, Vec<String>> = HashMap::new();
    let mut valid_hostnames: Vec<String> = Vec::new();
    for (hostname, hrefs) in link_hostnames.clone() {
        //println!("{}",hostname);
        if matcher.is_match(&hostname) {
            valids.insert(hostname, hrefs);
        } else {
            invalids.insert(hostname, hrefs);
        }
    }
    for (subhostname, mut subhrefs) in invalids.clone() {
        let subregex = regex::Regex::new(&subhostname).unwrap();
        for (hostname, mut hrefs) in valids.clone() {
            if subregex.is_match(&hostname) {
                hrefs.append(&mut subhrefs);
            }
        }
    }
    if valids.is_empty() {
        for (hostname, hrefs) in invalids.clone() {
            let subregex = regex::Regex::new(&hostname);
        }
    }

    for (hostname, mut hrefs) in valids.to_owned() {
        hrefs.sort_unstable();
        hrefs.dedup();
        // println!("{} : {:?}", hostname, hrefs);
    }
    everything.insert("valids".to_string(), valids);
    everything.insert("invalids".to_string(), invalids);
    everything
}

