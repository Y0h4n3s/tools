use std::process::*;

pub async fn populate_subs(root_domain: String, asn: Option<usize>, cidr: Option<String>) {

    let mut cmd = Command::new("amass");
    cmd.arg("enum")
        .arg("-d")
        .arg(root_domain);
    let out = cmd.output().expect("Amass Failed me");
    debug!("Amass is done: {:?}", out)

}