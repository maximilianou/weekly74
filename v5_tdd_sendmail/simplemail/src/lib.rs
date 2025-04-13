
use std::error::Error;

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub struct SimpleEmail {
  pub from: String,
  pub reply_to: String,
  pub to: String,
  pub subject: String,
  pub header: String,
  pub body: String,
}
pub struct SimpleCredentials {
  pub usr: String,
  pub pwd: String,
}
pub struct SimpleMailer {
  pub smtp: String,
}

pub struct SimpleMailConfig {
  pub email: SimpleEmail,
  pub credentials: SimpleCredentials,
  pub mailer: SimpleMailer,
}

impl SimpleMailConfig {
    pub fn build(args: &[String] ) -> Result<SimpleMailConfig, &'static str> {

        Ok( SimpleMailConfig {} ) 
    }
}

pub fn simple_send(config : SimpleMailConfig) -> Result<(), Box<dyn Error>>{
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_send_config(){
        let config = SimpleMailConfig::build(
          ["admin@simpledoers.work",
          "admin@simpledoers.work",
          "dev@simpledoers.work",
          ContentType::TEXT_PLAIN,
          "Messaggio TDD in rust simplemail subject",
          "Messaggio TDD in rust simplemail body"
          ]
        );
        assert_eq!((), simple_send(config.expect("expect some config")).expect("The email is set ok"));
    }
 }