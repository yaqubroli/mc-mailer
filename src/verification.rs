
use openssl::{hash::*};
use rand::{thread_rng, Rng};
use base64::engine::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::secrets::SPECIAL_SALT_CODE;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequestProto {
    pub(crate) email: String,
    pub(crate) minecraft_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WrittenReasonRequest {
    pub(crate) email: String,
    pub(crate) minecraft_username: String,
    pub(crate) reason: String,
}

#[derive(Debug)]
pub struct VerificationRequest {
    pub(crate) email: String,
    pub(crate) minecraft_username: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) seed: u32,
}

#[derive(Debug)]
pub struct VerificationReceipt {
    pub(crate) valid: bool,
    pub(crate) minecraft_username: String,
}

pub trait VerificationCodeGenerator {
    fn as_code(&self) -> String;
}

pub trait VerificationCodeValidator {
    fn from_code(code: String) -> VerificationReceipt;
}

pub trait VerificationCodeHydrator {
    fn hydrate(&self) -> VerificationRequest;
}

impl VerificationCodeGenerator for VerificationRequest {
    fn as_code(&self) -> String {
        println!("generating verification code from request: {:?}", self.clone());
        return format!(
                "{}{}{}",
                BASE64.encode(
                    &self.seed.to_be_bytes()
                ),
                BASE64.encode(
                    hash(MessageDigest::sha256(),
                    format!(
                        "{}{}{}{}",
                        self.seed,
                        self.date.format("%Y%m%d").to_string(),
                        SPECIAL_SALT_CODE,
                        self.minecraft_username
                    ).as_bytes()).unwrap().as_ref()
                ),
                BASE64.encode(
                    &self.minecraft_username.as_bytes()
                )
        );
    }
}

impl VerificationCodeValidator for VerificationReceipt {
    fn from_code(code: String) -> VerificationReceipt {
        println!("code: {}", code);
        println!("reconstructed username: {}", String::from_utf8(BASE64.decode(&code[49..]).unwrap()).unwrap());
        println!("reconstructed seed: {}", u32::from_be_bytes(BASE64.decode(&code[0..6]).unwrap().try_into().unwrap()));
        let username = String::from_utf8(BASE64.decode(&code[49..]).unwrap()).unwrap();
        return VerificationReceipt {
            minecraft_username: username.clone(),
            valid: code == VerificationRequest {
                email: String::from(""),
                minecraft_username: username,
                date: Utc::now(),
                seed: u32::from_be_bytes(BASE64.decode(&code[0..6]).unwrap().try_into().unwrap()),
            }.as_code()
        };
    }
}

impl VerificationCodeHydrator for VerificationRequestProto {
    fn hydrate(&self) -> VerificationRequest {
        return VerificationRequest {
            email: self.email.clone(),
            minecraft_username: self.minecraft_username.clone(),
            date: Utc::now(),
            seed: rand::thread_rng().gen::<u32>(),
        };
    }
}