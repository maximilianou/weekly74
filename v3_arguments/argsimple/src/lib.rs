use std::env;

struct QueryText{
  query: String,
  file_path: String,
}
pub fn parameter(){
    println!(" -- parameter() -- ");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!(" -- parameter() -- ");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duck";
        let contents = "Rust: safe, fast, productive. Pick three. Duct tape.";
        assert_eq!(vec!["save, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let contents = "Rust: safe, fast, productive. Pick three. Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}