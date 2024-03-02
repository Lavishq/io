use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "searching for {} in file {}",
        config.query, config.file_path
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("application err: {e}");
        process::exit(1);
    }
}
