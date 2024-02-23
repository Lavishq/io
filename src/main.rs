use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("searching for {} in file {}", query, file_path);
    let content = fs::read_to_string(file_path).expect("should be able to redd");
    println!("text: \n {content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
