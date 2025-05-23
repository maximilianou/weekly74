// src/lib.rs
pub mod email_fp {

  use std::fmt;
  use regex::Regex;
  use lazy_static::lazy_static;
  use config::Config;
  use lettre::transport::smtp::authentication::Credentials;
  use lettre::SmtpTransport;
  use lettre::Transport;
  use lettre::Message;
  use lettre::message::header::ContentType;
  



  #[derive(Debug, PartialEq)]
  pub struct SimpleEmail {
    pub from: String,
    pub reply_to: String,
    pub to: String,
    pub header_content_type: ContentType,
    pub subject: String,
    pub body: String,
  }

  lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
               .expect("Failed to create email regex");
  }

  impl SimpleEmail {
    pub fn build(
      from: String,
      reply_to: String,
      to: String,
      header_content_type: ContentType,
      subject: String,
      body: String,
    ) -> Result<Self, String>{

      let validate_email = |email: &str, field_name: &str| -> Result<(), String>{
        if EMAIL_REGEX.is_match(email){
          Ok(())
        }else{
          Err(format!("Invalid email format for email '{}': '{}'", field_name, email))
        }
      };

      validate_email(&from, "from")?;
      validate_email(&reply_to, "reply_to")?;
      validate_email(&to, "to")?;
      Ok( SimpleEmail{
        from,
        reply_to,
        to,
        header_content_type,
        subject,
        body,
      })
    }
    
  }

  pub fn send_email_functionality<F>(email: SimpleEmail, sender: F) 
         -> Result<(), String>
  where  F: Fn(&SimpleEmail) -> Result<(), String>,
  {
    println!("Preparing email for sending..");
    let send_result = sender(&email);
    match send_result{
      Ok(_) => println!("Sender function reported success."),
      Err(ref e) => println!("Sender function reported failure {}", e), 
    }
    send_result
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



  fn simulate_smtp_send(email: &SimpleEmail) -> Result<(), String> {
    let mailer = SimpleMailer::build().unwrap();
      let creds = Credentials::new(mailer.credentials.usr.to_owned(), 
      mailer.credentials.psw.to_owned());
  
      // Open a remote connection to gmail
      let smtp_mailer = SmtpTransport::relay(&mailer.smtp)
          .unwrap()
          .credentials(creds)
          .build();
  

          let message = Message::builder()
          .from(email.from.parse().unwrap())
          .reply_to(email.reply_to.parse().unwrap())
          .to(email.to.parse().unwrap())
          .subject(email.subject)
          .header(email.header_content_type)
          .body(email.body)
          .unwrap();


      // Send the email
    match smtp_mailer.send(&message) {
      Ok(_) => {
          println!("Email sent successfully!"); 
          Ok("Email sent successfully!".to_string());
      },
      Err(e) => panic!("Could not send email: {e:?}"),
    }  
    Ok(())
  }


  impl SimpleMailer {
    pub fn build() -> Result<SimpleMailer, &'static str> {
        let settings = Config::builder()
            // Add in `./settings.yaml`
            .add_source(config::File::with_name("settings"))
            .add_source(config::File::with_name(".env"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();
        // config from filesystem
        let sc = SimpleCredentials {
            usr: settings.get("sm_cred_usr").unwrap(),
            psw: settings.get("sm_cred_psw").unwrap(),
        };
        Ok(SimpleMailer {
          credentials: sc,
          smtp: settings.get("sm_smtp").unwrap(),
        })
    }
}

}


// tests/email_test.rs
// cargo test --test email_test
#[cfg(test)]
mod tests {
  use crate::email_fp::*;
  #[test]
  fn test_content_type_display(){
    assert_eq!(format!("{}", ContentType::TEXT_PLAIN), "text/plain");
  }  
  #[test]
  fn test_simple_email_new_valid(){
    let from = "sender@simpledoers.work".to_string();
    let reply_to = "reply@simpledoers.work".to_string();
    let to = "recipient@simpledoers.work".to_string();
    let content_type = ContentType::TextPlain;
    let subject = "Test Subject".to_string();
    let body = "Test Body".to_string();
    let email_result = SimpleEmail::build(
        from.clone(),
        reply_to.clone(),
        to.clone(),
        content_type,
        subject.clone(),
        body.clone(),
    );
    assert!(email_result.is_ok());
    let email = email_result.unwrap();
    assert_eq!(email.from, from);
    assert_eq!(email.reply_to, reply_to);
    assert_eq!(email.to, to);
//    assert_eq!(email.header_content_type, content_type);
    assert_eq!(email.subject, subject);
    assert_eq!(email.body, body);
  }

  #[test]
  fn test_simple_email_new_invalid_from(){
    let from = "invalid-email".to_string();
    let reply_to = "reply_simpledoers.work".to_string();
    let to = "recipient@simpledoers.work".to_string();
    let content_type = ContentType::TextPlain;
    let subject = "Test Subject".to_string();
    let body = "Test Body".to_string();
    let email_result = SimpleEmail::build(
        from.clone(),
        reply_to,
        to,
        content_type,
        subject,
        body,
    );
    assert!(email_result.is_err());
    let error_message = email_result.unwrap_err();
    assert!(error_message.contains("Invalid email format for email 'from'"));
    assert!(error_message.contains(&from));
  }


  #[test]
  fn test_email_functionality_success() {
    let email = SimpleEmail::build(
        "sender@simpledoers.work".to_string(),
        "reply@simpledoers.work".to_string(),
        "recipient@simpledoers.work".to_string(),
        ContentType::TextPlain,
        "Test Subject".to_string(),
        "Test Body".to_string(),
    ).unwrap();
    let send_result = send_email_functionality(email, simulate_smtp_send_success);
    assert!(send_result.is_ok());
  }

  fn simulate_smtp_send_success(email: &SimpleEmail) -> Result<(), String> {
    println!("\n --- Simulating successful email send --- ");
    println!("From: {}", email.from);
    println!(" ----------------------------------------");
    Ok(())
  }
  fn simulate_smtp_send_failure(email: &SimpleEmail) -> Result<(), String>{
    println!("\n --- Simulating failed email send --- ");
    println!(" Attempting to send email to: {}", email.to);
    println!(" ------------------------------------ ");
    Err("Simulated SMTP connection error".to_string())
  } 

  #[test]
  fn test_send_email_functionality_failure(){
    let email = SimpleEmail::build(
        "sender@simpledoers.work".to_string(),
        "reply@simpledoers.work".to_string(),
        "recipient@simpledoers.work".to_string(),
        ContentType::TextPlain,
        "The Subject".to_string(),
        "The Body".to_string(),
    ).unwrap();
    let send_result = send_email_functionality(email, simulate_smtp_send_failure);
    assert!(send_result.is_err());
    let error_message = send_result.unwrap_err();
    assert_eq!(error_message, "Simulated SMTP connection error");
  }





}




