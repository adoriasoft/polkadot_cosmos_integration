use std::vec::Vec;

pub fn get_something() -> Vec<u8> {
    let mut tmp = Vec::new();
    tmp.push(1u8);
    let body = reqwest::blocking::get("https://www.rust-lang.org")
        .unwrap()
        .text()
        .unwrap();
    println!("body = {:?}", body);
    tmp
}
