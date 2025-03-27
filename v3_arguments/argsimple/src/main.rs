use std::env;
use std::fs;
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

fn main() {
  //first_step();
  second_step();
}

