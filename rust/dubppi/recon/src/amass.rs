use std::process::*;
use organizer::helpers::*;
use organizer::models::DumpCollector;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use consumer::helpers::parsers::base_64_me;

pub async fn populate_subs(root_domain: String, asn: Option<String>, cidr: Option<String>, config: Option<String>, pool: &PooledConnection<ConnectionManager<PgConnection>>) {

    let mut cmd = Command::new("amass");
    cmd.arg("enum")
        .arg("-d")
        .arg(root_domain)
        .arg("-brute")
        .arg("-active")
        .arg("p")
        .arg("80,443,8080,8000")
        .arg("-wm");
    match asn {
        Some(val) => {cmd.arg("-asn").arg(val);},
        None => ()
    }
    match cidr {
        Some(val) => {cmd.arg("-cidr").arg(val);},
        None => ()
    }
    match config {
        Some(val) => {cmd.arg("-config").arg(val);},
        None => ()
    }



    let out = cmd.output().expect("[-] Amass Failed me");
    let subs = String::from_utf8_lossy(&out.stdout);
    let subs = subs.split("\n").collect::<Vec<&str>>();
    debug!("Amass is done: {:?}", &subs);
    println!("[+] Amass is done Subs Discovered: {:?}", &subs);
    println!("[+] Saving Subdomains To Database");
    debug!("Saving Subdomains To Database");

    for sub in subs {
        let insertable = DumpCollector {
            id: 0,
            hostname: Option::from(sub.to_string()),
            full_path: None,
            protocol: None,
            path_only: None,
            full_params: None,
            href: None,
            path_href: None,
            link_from: None,
            ip: None,
            port: None,
            endpoint_id: base_64_me("/tools/amass")
        };
        insert_much_data_subdomains(&insertable, pool);
    }


}