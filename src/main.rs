use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Data {
    id: i32,
    first_name: String,
    last_name: String,
    gender: String,
    ip_address: String,
}

fn main() {
    let test_string = "Hello World";
    println!("{}", test_string);
    
    read_mock();
}

fn read_mock() -> Vec<Data>{
    let file_content = fs::read_to_string("MOCK_DATA.json")
        .expect("Can't read file");

    let data: Vec<Data> = serde_json::from_str(&file_content)
        .expect("Can't read JSON");

    data
}