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

pub fn simple_send() -> Result<(), Box<dyn Err>>{
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_send(){

        simple_send();
    }
}