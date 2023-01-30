use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let Config { query, file_path, ignore_case: _ } = &config;

    println!("Searching for \"{}\"", query);
    println!("In file \"{}\"", file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
