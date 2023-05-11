pub mod verification {
    use openssl::hash::*;
    use base64::engine::Engine as _;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64;
    use chrono::prelude::*;
    use crate::SPECIAL_SALT_CODE;

    #[derive(Debug)]
    pub struct VerificationRequest {
        pub(crate) minecraft_username: String,
        pub(crate) date: DateTime<Utc>,
        pub(crate) seed: u32,
    }
    
    #[derive(Debug)]
    pub struct VerificationReceipt {
        valid: bool,
        minecraft_username: String,
    }
    
    pub trait VerificationCodeGenerator {
        fn as_code(&self) -> String;
    }
    
    pub trait VerificationCodeValidator {
        fn from_code(code: String) -> VerificationReceipt;
    }

    impl VerificationCodeGenerator for VerificationRequest {
        fn as_code(&self) -> String {
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
                            self.date.timestamp(),
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
            let username = String::from_utf8(BASE64.decode(&code[49..]).unwrap()).unwrap();
            return VerificationReceipt {
                minecraft_username: username.clone(),
                valid: code == VerificationRequest {
                    minecraft_username: username,
                    date: Utc::now(),
                    seed: u32::from_be_bytes(BASE64.decode(&code[0..6]).unwrap().try_into().unwrap()),
                }.as_code()
            };
        }
    }
}