use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
pub fn paste_to_file(data: HashMap<String, Vec<String>>) -> bool{
    let file_name = &data.get("file-name").unwrap()[0];
    let file_data = data.get("file-data").unwrap();
    let mut file = OpenOptions::new().append(true).create(true).open(file_name).unwrap();
    println!("{} saving", file_name);
    let mut save_check = false;
    let mut count = 0;
    loop {
        if save_check || count == 5{
            break;
        }
        save_check = file.write(to_file_data(file_data).as_bytes()).is_ok();
        count += 1;
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
