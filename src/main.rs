use chrono::Utc;
use lettre_email::EmailBuilder;
use secrets::secrets::EMAIL_USERNAME;
use crate::verification::verification::*;
use crate::secrets::secrets::SPECIAL_SALT_CODE;

pub mod verification;
pub mod secrets;

async fn send_verification_email(verificationRequest: VerificationRequest, ) -> impl Responder {
    let email = EmailBuilder::new()
        .to(format("{}@st-andrews.ac.uk", verificationRequest.email))
        .from(EMAIL_USERNAME)
        .subject("Minecraft Verification")
        .text(format!("Here's your whitelist verification link: https://mc.7800.io/verify/{}", verificationRequest.as_code()))
        .build();
}

fn main() {
    let verification_request = VerificationRequest {
        minecraft_username: String::from("jacobroly"),
        date: Utc::now(),
        seed: 45343,
    };
    println!("Generated verification request: {:?}", verification_request);
    let verification_code = verification_request.as_code();
    println!("Generated verification code: {}", verification_code);
    let verification_receipt = VerificationReceipt::from_code(verification_code);
    println!("Generated verification receipt: {:?}", verification_receipt);
}
