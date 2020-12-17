use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let mut json: &[u8] = &arg[1].as_bytes();
    let index: &usize = &arg[2].parse::<usize>().unwrap();
    if index < &json.len() { 
        if index == &0 && json.len() > 1{
            &json[1] = &String::from("$").as_bytes()[0];
        }
        else if *index == json.len() - 1 && json.len() > 1{
            json[json.len() - 2] = String::from("$").as_bytes()[0];
        }
        else {
            json[index - 1] = String::from("$").as_bytes()[0];
            json[index + 1] = String::from("$").as_bytes()[0];
        }
    }

    println!("{}",String::from_utf8_lossy(json));
}
