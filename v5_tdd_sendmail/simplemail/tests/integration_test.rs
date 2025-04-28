// simplemail/tests/integration_test.rs
// cargo test --test integration_test
#[cfg(test)]
mod tests {
//    use super::*;
    use simplemail::simplemail::*;
    #[test]
    fn test_i_config(){
        let config = SimpleMailConfig::build().unwrap();
        assert_eq!("Admin <admin@simpledoers.work>", config.email.from);
    }
    
    #[test]
    fn test_i_send(){
      let config = SimpleMailConfig::build();
      assert_eq!((), simple_send(config.unwrap()).unwrap());
    }
 }
