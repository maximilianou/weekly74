use simplemail::SimpleMailConfig;
fn main() {

    println!("--- main() --- ");
    let smc = SimpleMailConfig::build();
    println!("{:#?}", &smc);
    println!("--- main() --- ");

}
