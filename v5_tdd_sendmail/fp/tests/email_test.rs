// tests/email_test.rs
// cargo test --test email_test
#[cfg(test)]
mod tests {
  use fp::email_fp::*;
  #[test]
  fn test_content_type_display(){
    assert_eq!(format!("{}", ContentType::TextPlain), "text/plain");
    assert_eq!(format!("{}", ContentType::TextHtml ), "text/html");
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

  
}
