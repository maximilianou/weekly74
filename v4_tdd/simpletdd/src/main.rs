// src/main.rs
use std::env;
use std::process;
use simpletdd::Config;
fn thirteenth_step(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing argments: {err}");
    process::exit(1);
  } );
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);
  if let Err(e) = simpletdd::run(config) {
    println!("Application error {e}");
    process::exit(1);
  }
}

fn main() {
    thirteenth_step();
}
