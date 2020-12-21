extern crate regex;


use crate::dbmodels::SubDomain;
use std::collections::HashMap;

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
        let the_useful_data = request_json[2].clone();
        //TODO implement choice filtering by hostname
        println!("{:?}", self.data);
        let hostnames: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> =
           get_root_hostnames(the_useful_data.clone(), self.root_domain.clone());
        //let hostnames_entry: HashMap<String, HashMap<String, Vec<String>>> =
        //    one_love(hostnames.clone(), self.root_domain.clone());



        let link_dirs = request_json[2].clone();
        //let wordlists = get_wordlist_all(link_dirs);

        Option::from(hostnames)
    }
}

fn one_love(domain_hrefs: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, root: String)
             {

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

fn get_hostnames(hrefs: HashMap<String,Vec<String>>)
                 -> HashMap<String,HashMap<String, Vec<String>>> {
    let matcher = regex::Regex::new(r".?(?:https?:)?//([\w\-.]+).?").unwrap();
    let mut host_href_dirs: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for (mut href, dirs)  in hrefs.to_owned() {
        if href.starts_with("//") {
            href = format!("http{}", href);
        }
                    //println!("{:?}", &cap[1]);
            let hostname = get_hostname(&href);
            let mut href_dirs: HashMap<String, Vec<String>> = HashMap::new();
            href_dirs.insert(href.clone(), dirs.to_owned());
            host_href_dirs.insert(hostname.parse().unwrap(), href_dirs);

    }
    host_href_dirs
}

fn get_root_hostnames(hrefs: HashMap<String, Vec<String>>, root_domain: String)
                      -> HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> {
    let link_hostnames = get_hostnames(hrefs).clone();
    //println!("theud: {:?}", link_hostnames);

    let re = format!(r"(^{})", &root_domain);
    //println!("Regex: {} {}", re, root_domain);
    let mut everything: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> = HashMap::new();
    let matcher = regex::Regex::new(&re).unwrap();
    let mut valids: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut invalids: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for (hostname, hrefs) in link_hostnames.clone() {
        //println!("{}",hostname);
        if matcher.is_match(&hostname) {
            valids.insert(hostname, hrefs);
        } else {
            invalids.insert(hostname, hrefs);
        }
    }
    if valids.is_empty() {
        for (hostname, _hrefs) in invalids.clone() {
            let _subregex = regex::Regex::new(&hostname);
        }
    }

    for (hostname, mut hrefs) in valids.to_owned() {
         //println!("{} : {:?}", hostname, hrefs);
    }

    everything.insert("valids".to_string(), valids);
    everything.insert("invalids".to_string(), invalids);
    everything
}


