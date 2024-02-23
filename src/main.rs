use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {} in file {}", query, file_path);
    let content = fs::read_to_string(file_path).expect("should be able to redd");
    println!("text: \n {content}");
}
