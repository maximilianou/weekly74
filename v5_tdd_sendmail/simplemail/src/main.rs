//simplemail/src/main.rs
use simplemail::simplemail::SimpleMailConfig;
use simplemail::simplemail::simple_send;
fn main() {

    println!("--- main() --- ");
    let config = SimpleMailConfig::build();
    let smc = simple_send(config.unwrap()).unwrap();
    assert_eq!{"Email sent successfully!", &smc};
    println!("{:#?}", &smc);
    println!("--- main() --- ");

}
