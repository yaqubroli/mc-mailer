use actix_web::Responder;
use lettre::message::Mailbox;
use lettre::*;
use lettre::address::Address;
use lettre::transport::smtp::authentication::Credentials;
use lettre_email::EmailBuilder;
use crate::verification::*;
use crate::secrets::*;

pub async fn send_verification_email(request: VerificationRequest) -> Result<(), String> {

    // TODO: Move creds and mailer outside of the function; maybe just make them static?

    let creds = Credentials::new(
        format!("{}@{}", EMAIL_USERNAME, EMAIL_DOMAIN),
        EMAIL_PASSWORD.to_string()
    );

    let mailer = SmtpTransport::starttls_relay("mail.cock.li")
        .unwrap()
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(
            Mailbox::new(
                Some(VERIFY_EMAIL_NICKNAME.to_string()),
                Address::new(EMAIL_USERNAME.to_string(), EMAIL_DOMAIN.to_string()).unwrap()
            )
        )
        .to(
            Mailbox::new(
                Some(request.minecraft_username.clone()),
                Address::new(request.email.clone(), "st-andrews.ac.uk").unwrap()
            )
        )
        .subject(VERIFY_EMAIL_SUBJECT)
        .body(
            VERIFY_EMAIL_BODY_1.to_string() + &request.as_code() + VERIFY_EMAIL_BODY_2
        )
        .unwrap();
    println!("Sending email...");
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to send email: {}", e))
    }
}

pub async fn send_written_reason_email(request: WrittenReasonRequest) -> Result<(), String> {
    let creds = Credentials::new(
        format!("{}@{}", EMAIL_USERNAME, EMAIL_DOMAIN),
        EMAIL_PASSWORD.to_string()
    );

    let mailer = SmtpTransport::starttls_relay("mail.cock.li")
        .unwrap()
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(
            Mailbox::new(
                Some(VERIFY_EMAIL_NICKNAME.to_string()),
                Address::new(EMAIL_USERNAME.to_string(), EMAIL_DOMAIN.to_string()).unwrap()
            )
        )
        .to(
            Mailbox::new(
                Some(request.minecraft_username.clone()),
                Address::new("walchuk2018", "icloud.com").unwrap()
            )
        )
        .subject("Request from ".to_string() + &request.minecraft_username)
        .body(
            format!("{:?}", request)
        )
        .unwrap();
    println!("Sending email...");
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to send email: {}", e))
    }
}