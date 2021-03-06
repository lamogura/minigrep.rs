extern crate minigrep;

use minigrep::Config;
use std::env;
use std::process;

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  minigrep::run(config).unwrap_or_else(|err| {
    eprintln!("Problem searching text: {}", err);
    process::exit(1);
  });
}
