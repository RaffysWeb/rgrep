use rgrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1)
    });

    println!("args: {}, {}", config.query, config.file_path);
    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
