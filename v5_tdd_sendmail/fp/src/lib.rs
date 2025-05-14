// src/lib.rs
pub mod email_fp {

  use std::fmt;
  use regex::Regex;
  use lazy_static::lazy_static;

  #[derive(Debug, PartialEq)]
  pub enum ContentType {
    TextPlain,
    TextHtml,
  }


  impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::TextHtml  => write!(f, "text/html"),
        }
    }
  }

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

}