// simplemail/src/lib.rs
pub mod simplemail {
    use config::Config;
//    use std::collections::HashMap;
    use std::error::Error;

    use lettre::{
        message::header::ContentType, transport::smtp::authentication::Credentials, Message,
        SmtpTransport, Transport,
    };

    #[derive(Debug)]
    pub struct SimpleEmail {
        pub from: String,
        pub reply_to: String,
        pub to: String,
        pub header_content_type: ContentType,
        pub subject: String,
        pub body: String,
    }
    #[derive(Debug)]
    pub struct SimpleCredentials {
        pub usr: String,
        pub psw: String,
    }
    #[derive(Debug)]
    pub struct SimpleMailer {
        pub credentials: SimpleCredentials,
        pub smtp: String,
    }

    #[derive(Debug)]
    pub struct SimpleMailConfig {
        pub email: SimpleEmail,
        pub mailer: SimpleMailer,
    }

    impl SimpleMailConfig {
        pub fn build() -> Result<SimpleMailConfig, &'static str> {
            let settings = Config::builder()
                // Add in `./settings.yaml`
                .add_source(config::File::with_name("settings"))
                .add_source(config::File::with_name(".env"))
                // Add in settings from the environment (with a prefix of APP)
                // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
                .add_source(config::Environment::with_prefix("APP"))
                .build()
                .unwrap();

            // argument from the command line
            // TEST syntactic email
            // TEST syntactic subject long
            let se = SimpleEmail {
                from: settings.get("sm_from").unwrap(),
                reply_to: settings.get("sm_reply_to").unwrap(),
                to: settings.get("sm_to").unwrap(),
                header_content_type: ContentType::TEXT_PLAIN,
                subject: settings.get("sm_subject").unwrap(),
                body: settings.get("sm_body").unwrap(),
            };

            // config from filesystem
            let sc = SimpleCredentials {
                usr: settings.get("sm_cred_usr").unwrap(),
                psw: settings.get("sm_cred_psw").unwrap(),
            };

            // config from filesystem
            let sm = SimpleMailer {
                credentials: sc,
                smtp: settings.get("sm_smtp").unwrap(),
            };

            Ok(SimpleMailConfig {
                email: se,
                mailer: sm,
            })
        }
    }

    pub fn simple_send(config: SimpleMailConfig) -> Result<String, Box<dyn Error>> {
      tracing_subscriber::fmt::init();

      let email = Message::builder()
          .from(config.email.from.parse().unwrap())
          .reply_to(config.email.reply_to.parse().unwrap())
          .to(config.email.to.parse().unwrap())
          .subject(config.email.subject)
          .header(config.email.header_content_type)
          .body(String::from(config.email.body))
          .unwrap();
  
      let creds = Credentials::new(config.mailer.credentials.usr.to_owned(), 
      config.mailer.credentials.psw.to_owned());
  
      // Open a remote connection to gmail
      let mailer = SmtpTransport::relay(&config.mailer.smtp)
          .unwrap()
          .credentials(creds)
          .build();
  
      // Send the email
      match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!"); 
            Ok("Email sent successfully!".to_string())
        },
        Err(e) => panic!("Could not send email: {e:?}"),
      }
    }
}

/* 
// cargo test --lib
#[cfg(test)]
mod tests {
    use crate::simplemail::*;
    #[test]
    fn test_config(){
        let config = SimpleMailConfig::build().unwrap();
        assert_eq!("Admin <admin@simpledoers.work>", config.email.from);
    }
    #[test]
    fn test_send(){
      let config = SimpleMailConfig::build();
      assert_eq!((), simple_send(config.unwrap()).unwrap());
    }
 }
*/