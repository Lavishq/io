use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing args: {err}");
        process::exit(1);
    });

    println!(
        "searching for {} in file {}",
        config.query, config.file_path
    );
    let content = fs::read_to_string(config.file_path).expect("should be able to redd");
    println!("text: \n {content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enuggh args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
