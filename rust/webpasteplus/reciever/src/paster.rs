use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn paste_to_file(
    data: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
    file_path: &String) -> bool {

    let save_check = false;
    let wordlists = data.get("wordlists").unwrap().to_owned();
    //let hostnames = data.get("hostnames").unwrap().get("hostnames").unwrap().get("hostnames").unwrap().to_owned();
    for (path, data) in wordlists {
        //println!("{}", path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(format!("{}/{}", file_path, data.get("hostname").unwrap()[0].to_owned()))
            .unwrap();
        file.write(to_file_data(data.get("exact").unwrap()).as_bytes()).unwrap();
    }

    save_check
}

fn to_file_data(data: &Vec<String>) -> String {
    let mut data_holder = String::with_capacity(1024);
    for d in data.iter() {
        data_holder.push_str(d);
        data_holder.push('\n')
    };
    data_holder
}
