pub mod matchers {
    pub fn domain_matches_by_hostname(hotname: &str, root_domain: &str) -> bool {
        let re = format!(".?{}.?", root_domain);
        let matcher = regex::Regex::new(&re).unwrap();
        matcher.is_match(hotname)
    }
}

pub mod parsers {
    use crate::models::request_models::{DomMuchData, MuchData};
    use regex::Match;
    use base64::encode;
    pub fn parse_link_into_dom_much_data(href: &str) -> DomMuchData{

        DomMuchData {
            data: vec![parse_link_into_much_data(href)],
            endpoint_id: "".to_string()
        }
    }

    pub fn parse_chunk_into_dom_much_data(href: &str) -> DomMuchData{
        let re =
            regex::Regex::new(r#".?((?:(https?|wss|ftp|ssh|smtp|rsync|git|file):?)//([\w\-.:@~]+))(([^\s\n"?#<']*)([?#;][^\n\s]*)?).?"#).unwrap();
        let mut coll = Vec::<MuchData>::new();
        debug!("Blob: {:?}", href);
         for caps in re.captures_iter(href) {
            coll.push(MuchData {
                full_link: match caps.get(0) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                link_only: match caps.get(1) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                protocol: match caps.get(2) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                port: 0,
                hostname: match caps.get(3) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                full_path: match caps.get(4) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                path_only: match caps.get(5) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                params: match caps.get(6) {
                    None => {"".to_string()}
                    Some(val) => {val.as_str().parse().unwrap_or("".to_string())}
                },
                page_from: "wayback".to_string()
            });
            debug!("Caps: {:?}", caps);

        };

        DomMuchData {
            data: coll,
            endpoint_id: base_64_me("/dom/much_data")
        }
    }

    pub fn parse_link_into_much_data(href: &str) -> MuchData {
        let re =
            regex::Regex::new(r#".?((?:(https?|wss|ftp|ssh|smtp|rsync|git|file):?)//([\w\-.]+))(([^\s\n"?#<']*)([?#;][^\n\s]*)?).?"#).unwrap();
        let matches = re.captures(href);
        match matches {
            Some(caps) => {
                //debug!("Caps: {:?}", caps);
            }
            None => {}
        }

        MuchData {
            full_link: "".to_string(),
            link_only: "".to_string(),
            protocol: "".to_string(),
            port: 0,
            hostname: "".to_string(),
            full_path: "".to_string(),
            path_only: "".to_string(),
            params: "".to_string(),
            page_from: "".to_string()
        }
    }
    // not tested
    pub fn get_port_from_link(href: &str) -> i32 {
        let port_re = regex::Regex::new(r#".?(?:http?|wss|ssh|ftp|file)*://([a-z0-9\-._~%!$&'()*+,;=]+@)?([a-z0-9\-._~%]+|\[[a-z0-9\-._~%!$&'()*+,;=:]+\]):([0-9]+)"#).unwrap();
        let port_match = port_re.captures(href).unwrap();
        let port = match port_match.get(3) {
            Some(port) => {
                port.as_str().parse::<i32>().unwrap()
            }
            _ => {
                match port_match.get(2) {
                    None => {0}
                    Some(protocol) => {
                        match protocol.as_str() {
                            "http" => 80,
                            "https" => 443,
                            _ => 0
                        }
                    }
                }
            }
        };
        port
    }

    pub fn base_64_me(dat: &str) -> String{
        encode(dat)
    }
}