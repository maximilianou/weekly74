use simplemail::SimpleMailConfig;
fn main() {

    println!("--- main() --- ");
    let smc = SimpleMailConfig::build().unwrap();
    println!("{:#?}", &smc);
//    println!( "{}", &smc);
    println!("--- main() --- ");

}
