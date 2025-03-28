// src/main
use std::env;
use std::fs;
/*
fn first_step(){
  let args: Vec<String> = env::args().collect();
  if args.len() > 2 {
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file {file_path}");    
  }else {
    println!("call with: simple [query] [file], please.");
  }
}
*/
/*
fn second_step(){
  let args: Vec<String> = env::args().collect();
  if args.len() > 2 {
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
                 .expect("Should have been able to read the file.");
    println!("With Text:\n{contents}");

  }else {
    println!("call with: simple [query] [file], please.");
  }
}
*/
/*
fn thirth_step(){
  let args: Vec<String> = env::args().collect();
  let ( query, file_path ) = parse_config(&args);
  println!("Searching for {query}");
  println!("In file {file_path}");
  let contents = fs::read_to_string(file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
fn parse_config(args: &[String]) -> (&str, &str){
  let query = &args[1];
  let file_path = &args[2];
  (query, file_path)
}
*/
/*
fn fourth_step(){
  let args: Vec<String> = env::args().collect();
  let config = parse_config(&args);
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
*/
/*
fn fifth_step(){
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args);
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
struct Config {
  query: String,
  file_path: String,
}
impl Config {
  fn new(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
  }
}
*/
/*
  fn sixth_step(){
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args);
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
struct Config {
  query: String,
  file_path: String,
}
impl Config {
  fn new(args: &[String]) -> Config{
    if args.len() < 3 {
      panic!("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
  }
}
*/
/*
fn seventh_step(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args);
  println!("Searching for {}", config.clone().unwrap().query);
  println!("In file {}", config.clone().unwrap().file_path);
  let contents = fs::read_to_string(config.clone().unwrap().file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
#[derive(Clone)]
struct Config {
  query: String,
  file_path: String,
}
impl Config {
  fn build(args: &[String]) -> Result<Config, &'static str>{
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok( Config { query, file_path } )
  }
}
*/
use std::process;
fn eigthth_step(){
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing argments: {err}");
    process::exit(1);
  } );

  println!("Searching for {}", config.clone().query);
  println!("In file {}", config.clone().file_path);
  let contents = fs::read_to_string(config.clone().file_path)
               .expect("Should have been able to read the file.");
  println!("With Text:\n{contents}");

}
#[derive(Clone)]
struct Config {
  query: String,
  file_path: String,
}
impl Config {
  fn build(args: &[String]) -> Result<Config, &'static str>{
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok( Config { query, file_path } )
  }
}

fn main() {
//  first_step();
//  second_step();
//  thirth_step();
//  fourth_step();
//  fifth_step();
//  sixth_step();
//  seventh_step();
eigthth_step();
}
