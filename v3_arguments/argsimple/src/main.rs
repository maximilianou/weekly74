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
fn main() {
//  first_step();
//  second_step();
//  thirth_step();
//  fourth_step();
fifth_step();
}
