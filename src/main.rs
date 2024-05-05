use minigrep::Config;
use std::{env, process::exit};
fn main() {
    let config = Config::from(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1)
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("{err}");
        exit(1)
    };
}
