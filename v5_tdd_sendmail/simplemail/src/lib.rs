
use std::error::Error;

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub struct SimpleEmail {
  pub from: String,
  pub reply_to: String,
  pub to: String,
  pub header_content_type: String,
  pub subject: String,
  pub body: String,
}
pub struct SimpleCredentials {
  pub usr: String,
  pub pwd: String,
}
pub struct SimpleMailer {
  pub credentials: SimpleCredentials,
  pub smtp: String,
}

pub struct SimpleMailConfig {
  pub email: SimpleEmail,
  pub mailer: SimpleMailer,
}

impl SimpleMailConfig {

    pub fn build(args: &[String] ) -> Result<SimpleMailConfig, &'static str> {


      let settings = Config::builder()
      // Add in `./settings.yaml`
      .add_source(config::File::with_name("settings"))
      .add_source(config::File::with_name(".env.json"))
      .add_source(config::File::with_name(".env"))
      // Add in settings from the environment (with a prefix of APP)
      // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
      .add_source(config::Environment::with_prefix("APP"))
      .build()
      .unwrap();
    

      // argument from the command line
      // TEST syntactic email
      // TEST syntactic subject long
      let se = SimpleEmail( 
        settings["sm_from"], 
        settings["sm_reply_to"], 
        settings["sm_to"], 
        ContentType::TEXT_PLAIN,
        settings["sm_subject"], 
        settings["sm_body"], 
      );

      // config from filesystem
      let sc = SimpleCredentials(
        settings["sm_cred_usr"], 
        settings["sm_cred_psw"]
      );

      
      // config from filesystem
      let sm = SimpleMailer(
        sc,
        settings["smtp.gmail.com"]
      );

        Ok( SimpleMailConfig {se, sm} )
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
    #[test]
    fn test_send_config_implicit(){
      let config = SimpleMailConfig::build();
      assert_eq!((), simple_send(config.expect("expect some config")).expect("The email is set ok"));

    }
 }