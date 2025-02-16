use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file: {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
