
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

async fn sending() {
    tracing_subscriber::fmt::init();

    let email = Message::builder()
        .from("Admin <admin@simpledoers.work>".parse().unwrap())
        .reply_to("Admin <admin@simpledoers.work>".parse().unwrap())
        .to("Dev <dev@simpledoers.work>".parse().unwrap())
        .subject("Hi, from rust smtp")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("asimpledoers@gmail.com".to_owned(), 
    "p2ssw0rd".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
