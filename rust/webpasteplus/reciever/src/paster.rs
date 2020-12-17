use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn paste_to_file(data: HashMap<String, Vec<String>>) -> bool {
    let mut save_check = false;
    let file_name: &String = &data.get("filename").unwrap()[0];
    for i in data.keys() {
        let mut file = OpenOptions::new().append(true).create(true)
            .open(file_name).unwrap();
        println!("{} saving", file_name);
        save_check = file.write(to_file_data(&data.get("data").unwrap()).as_bytes()).is_ok();

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
