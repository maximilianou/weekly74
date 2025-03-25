use std::env;

pub fn parameter(){
    println!(" -- parameter() -- ");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!(" -- parameter() -- ");
}