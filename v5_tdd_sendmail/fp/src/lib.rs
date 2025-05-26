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







/*


// src/lib.rs
pub mod email_fp {

    use std::fmt;
    use regex::Regex;
    use lazy_static::lazy_static;
    use config::Config;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport, Message};
    use lettre::message::header::ContentType;

    #[derive(Debug, PartialEq, Clone)] // Added Clone
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

    #[derive(Debug)]
    pub enum EmailError {
        Validation(String),
        Configuration(String),
        Parsing(String),
        Transport(String),
        Send(String),
    }

    impl fmt::Display for EmailError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                EmailError::Validation(s) => write!(f, "Validation Error: {}", s),
                EmailError::Configuration(s) => write!(f, "Configuration Error: {}", s),
                EmailError::Parsing(s) => write!(f, "Parsing Error: {}", s),
                EmailError::Transport(s) => write!(f, "Transport Error: {}", s),
                EmailError::Send(s) => write!(f, "Send Error: {}", s),
            }
        }
    }

    impl SimpleEmail {
        pub fn build(
            from: String,
            reply_to: String,
            to: String,
            header_content_type: ContentType,
            subject: String,
            body: String,
        ) -> Result<Self, EmailError> {
            let validate_field = |email_str: &str, field_name: &str| -> Result<(), EmailError> {
                if EMAIL_REGEX.is_match(email_str) {
                    Ok(())
                } else {
                    Err(EmailError::Validation(format!(
                        "Invalid email format for {}: '{}'",
                        field_name, email_str
                    )))
                }
            };

            validate_field(&from, "from")?;
            validate_field(&reply_to, "reply_to")?;
            validate_field(&to, "to")?;

            Ok(SimpleEmail {
                from,
                reply_to,
                to,
                header_content_type,
                subject,
                body,
            })
        }
    }

    pub fn send_email_functionality<F>(email: &SimpleEmail, sender: F)
                                        -> Result<(), EmailError>
        where F: Fn(&SimpleEmail) -> Result<(), EmailError>,
    {
        println!("Preparing email for sending...");
        let send_result = sender(email);
        match send_result {
            Ok(_) => println!("Sender function reported success."),
            Err(ref e) => println!("Sender function reported failure: {}", e),
        }
        send_result
    }

    #[derive(Debug, Clone)] // Added Clone
    pub struct SimpleCredentials {
        pub usr: String,
        pub psw: String,
    }

    #[derive(Debug, Clone)] // Added Clone
    pub struct SimpleMailerConfig {
        pub credentials: SimpleCredentials,
        pub smtp: String,
    }

    // Function to load mailer configuration
    pub fn load_mailer_config() -> Result<SimpleMailerConfig, EmailError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("settings").required(false))
            .add_source(config::File::with_name(".env").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .map_err(|e| EmailError::Configuration(format!("Failed to build config: {}", e)))?;

        let usr = settings.get_string("sm_cred_usr")
            .map_err(|e| EmailError::Configuration(format!("Missing/Invalid sm_cred_usr config: {}", e)))?;
        let psw = settings.get_string("sm_cred_psw")
            .map_err(|e| EmailError::Configuration(format!("Missing/Invalid sm_cred_psw config: {}", e)))?;
        let smtp_server = settings.get_string("sm_smtp")
            .map_err(|e| EmailError::Configuration(format!("Missing/Invalid sm_smtp config: {}", e)))?;

        Ok(SimpleMailerConfig {
            credentials: SimpleCredentials { usr, psw },
            smtp: smtp_server,
        })
    }
    
    // Function to create an SMTP transport
    pub fn create_smtp_transport(config: &SimpleMailerConfig) -> Result<SmtpTransport, EmailError> {
        let creds = Credentials::new(
            config.credentials.usr.clone(),
            config.credentials.psw.clone(),
        );
        SmtpTransport::relay(&config.smtp)
            .map_err(|e| EmailError::Transport(format!("Failed to create relay for '{}': {}", config.smtp, e)))?
            .credentials(creds)
            .build()
            .pipe(Ok) // SmtpTransportBuilder::build returns SmtpTransport directly
                      // If it could fail in a way not caught by relay(), this would need adjustment.
                      // For now, assuming build() after successful relay() and credentials() is fine.
    }


    // Function to build a lettre::Message from SimpleEmail
    pub fn build_lettre_message(email: &SimpleEmail) -> Result<Message, EmailError> {
        Message::builder()
            .from(email.from.parse().map_err(|e| EmailError::Parsing(format!("Invalid 'from' address '{}': {}", email.from, e)))?)
            .reply_to(email.reply_to.parse().map_err(|e| EmailError::Parsing(format!("Invalid 'reply_to' address '{}': {}", email.reply_to, e)))?)
            .to(email.to.parse().map_err(|e| EmailError::Parsing(format!("Invalid 'to' address '{}': {}", email.to, e)))?)
            .subject(email.subject.clone())
            .header(email.header_content_type.clone())
            .body(email.body.clone())
            .map_err(|e| EmailError::Send(format!("Failed to build email message: {}", e)))
    }

    // Higher-order function that returns a configured sender function
    pub fn actual_smtp_sender_closure_builder(
        config_loader: fn() -> Result<SimpleMailerConfig, EmailError>,
        transport_creator: fn(&SimpleMailerConfig) -> Result<SmtpTransport, EmailError>,
        message_builder: fn(&SimpleEmail) -> Result<Message, EmailError>,
    ) -> Box<dyn Fn(&SimpleEmail) -> Result<(), EmailError>> {
        Box::new(move |email_data: &SimpleEmail| {
            println!("Preparing email for sending via actual SMTP...");
            let config = config_loader().map_err(|e| {
                println!("Failed to load config: {}", e);
                e
            })?;
            let transport = transport_creator(&config).map_err(|e| {
                println!("Failed to create transport: {}", e);
                e
            })?;
            let message = message_builder(email_data).map_err(|e| {
                println!("Failed to build message: {}", e);
                e
            })?;

            transport.send(&message).map_err(|e| {
                let err_msg = format!("Actual SMTP send failed: {:?}", e);
                println!("{}", err_msg);
                EmailError::Send(err_msg)
            })?;
            println!("Email sent successfully via actual SMTP.");
            Ok(())
        })
    }

    // A concrete sender function using the components (replaces original simulate_smtp_send)
    pub fn configured_smtp_sender(email: &SimpleEmail) -> Result<(), EmailError> {
        let sender_logic = actual_smtp_sender_closure_builder(
            load_mailer_config,
            create_smtp_transport,
            build_lettre_message
        );
        sender_logic(email)
    }
}

// Helper for SmtpTransportBuilder::build() which returns SmtpTransport, not Result
// Trait to allow .pipe(Ok) or similar for cleaner chaining if needed
trait Pipeable: Sized {
    fn pipe<F, R>(self, f: F) -> R where F: FnOnce(Self) -> R {
        f(self)
    }
}
impl<T> Pipeable for T {}










// tests/email_test.rs
// cargo test --test email_test
#[cfg(test)]
mod tests {
    use super::email_fp::*; // Use the module name from lib.rs
    use lettre::message::header::ContentType; // Ensure ContentType is in scope

    #[test]
    fn test_content_type_display() {
        assert_eq!(format!("{}", ContentType::TEXT_PLAIN), "text/plain");
    }

    #[test]
    fn test_simple_email_build_valid() {
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
            content_type.clone(),
            subject.clone(),
            body.clone(),
        );
        assert!(email_result.is_ok());
        let email = email_result.unwrap();
        assert_eq!(email.from, from);
        assert_eq!(email.reply_to, reply_to);
        assert_eq!(email.to, to);
        assert_eq!(email.header_content_type, content_type);
        assert_eq!(email.subject, subject);
        assert_eq!(email.body, body);
    }

    #[test]
    fn test_simple_email_build_invalid_from() {
        let from = "invalid-email".to_string();
        let reply_to = "reply@simpledoers.work".to_string(); // valid reply_to for this test case
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
        match email_result.unwrap_err() {
            EmailError::Validation(msg) => {
                assert!(msg.contains("Invalid email format for from"));
                assert!(msg.contains(&from));
            }
            _ => panic!("Expected EmailError::Validation"),
        }
    }

    // Mock sender for success
    fn mock_sender_success(_email: &SimpleEmail) -> Result<(), EmailError> {
        println!("\n --- Mocking successful email send --- ");
        // println!("From: {}", _email.from); // _email is available if needed
        println!(" ----------------------------------------");
        Ok(())
    }

    // Mock sender for failure
    fn mock_sender_failure(_email: &SimpleEmail) -> Result<(), EmailError> {
        println!("\n --- Mocking failed email send --- ");
        // println!(" Attempting to send email to: {}", _email.to); // _email is available
        println!(" ------------------------------------ ");
        Err(EmailError::Send("Mocked SMTP connection error".to_string()))
    }

    #[test]
    fn test_send_email_functionality_success() {
        let email = SimpleEmail::build(
            "sender@simpledoers.work".to_string(),
            "reply@simpledoers.work".to_string(),
            "recipient@simpledoers.work".to_string(),
            ContentType::TextPlain,
            "Test Subject".to_string(),
            "Test Body".to_string(),
        ).unwrap();
        let send_result = send_email_functionality(&email, mock_sender_success);
        assert!(send_result.is_ok());
    }

    #[test]
    fn test_send_email_functionality_failure() {
        let email = SimpleEmail::build(
            "sender@simpledoers.work".to_string(),
            "reply@simpledoers.work".to_string(),
            "recipient@simpledoers.work".to_string(),
            ContentType::TextPlain,
            "The Subject".to_string(),
            "The Body".to_string(),
        ).unwrap();
        let send_result = send_email_functionality(&email, mock_sender_failure);
        assert!(send_result.is_err());
        match send_result.unwrap_err() {
            EmailError::Send(msg) => {
                assert_eq!(msg, "Mocked SMTP connection error");
            }
            _ => panic!("Expected EmailError::Send"),
        }
    }

    // Example of how you might test the configured_smtp_sender
    // This would require mocking the configuration file or environment variables
    // or providing mock implementations for load_mailer_config, create_smtp_transport etc.
    // For now, this test is conceptual.
    /*
    #[test]
    fn test_configured_smtp_sender_example() {
        // This test would typically require a more elaborate setup
        // to mock file I/O for config or environment variables.
        // Or, you'd test `actual_smtp_sender_closure_builder` by providing mock functions.

        // Set up mock environment variables for testing load_mailer_config if it reads from env
        std::env::set_var("APP_SM_CRED_USR", "testuser");
        std::env::set_var("APP_SM_CRED_PSW", "testpass");
        std::env::set_var("APP_SM_SMTP", "smtp.test.com");


        let email = SimpleEmail::build(
            "sender@example.com".to_string(),
            "reply@example.com".to_string(),
            "recipient@example.com".to_string(),
            ContentType::TextPlain,
            "Integration Test Subject".to_string(),
            "Integration Test Body".to_string(),
        ).unwrap();

        // In a real test, you might not call the actual configured_smtp_sender
        // but rather test its components or use a mock SMTP server.
        // The `actual_smtp_sender_closure_builder` provides a way to inject mock components:

        let mock_successful_transport_creator = |_config: &SimpleMailerConfig| {
            // Return a mock SmtpTransport or Ok with a dummy one if it's not used
            // For this example, assume we can construct a dummy SmtpTransport
            // or make create_smtp_transport more easily mockable.
            // This part is tricky without a mock SmtpTransport struct.
            // For now, let's assume it returns an error to show flow.
            Err(EmailError::Transport("Mock transport creation failed".to_string()))
        };
        
        let test_sender = email_fp::actual_smtp_sender_closure_builder(
            email_fp::load_mailer_config, // Uses actual config loading
            mock_successful_transport_creator, // Uses mock transport
            email_fp::build_lettre_message
        );

        let result = test_sender(&email);
        assert!(result.is_err());
        if let Err(EmailError::Transport(msg)) = result {
            assert_eq!(msg, "Mock transport creation failed");
        } else {
            panic!("Expected Transport error");
        }
        
        // Clean up env vars
        std::env::remove_var("APP_SM_CRED_USR");
        std::env_remove_var("APP_SM_CRED_PSW");
        std::env_remove_var("APP_SM_SMTP");
    }
    */
}
*/