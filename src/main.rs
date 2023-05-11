use chrono::Utc;
use crate::verification::verification::*;
use crate::secrets::secrets::SPECIAL_SALT_CODE;

pub mod verification;
pub mod secrets;

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
