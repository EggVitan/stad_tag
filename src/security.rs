//src/security.rs
extern crate argon2;
use actix_web::HttpRequest;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt, Value
    },
    Argon2
};

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde_json::json;
use sha2::Sha384;
use std::{collections::BTreeMap, string};

pub enum errors {
    user_exists(String),
    jwt_invalid(String),
}

static ARGON2_PEPPER: &[u8] = &[0x38, 0xcd, 0x0d, 0x9a, 0x13, 0x8a, 0x52, 0x97, 0xa5, 0x61, 0xae, 0x15];
static JWT_SECRET: &[u8] = "safasfasfasfaffasfasfas".as_bytes();
pub fn hash_pwd(pwd: &str, salt: SaltString) -> (String) { // (pwd, salt)
    argon2::ParamsBuilder::new();

    let secret: &[u8] = &[0x38, 0xcd, 0x0d, 0x9a, 0x13, 0x8a, 0x52, 0x97, 0xa5, 0x61, 0xae, 0x15,
    0xab, 0xe1, 0x2c, 0xe0];
    let mut hasher: Argon2;
        
    match Argon2::new_with_secret(
        secret,
        argon2::Algorithm::Argon2id, 
        argon2::Version::V0x13, 
        argon2::Params::default(),
    ){
        Ok(i) => {
            hasher = i;
        }
        Err(error) => {
            println!("Error: {:?}", error);
            return "".to_string();
        }
    }
    let hash: PasswordHash;
    match hasher.hash_password(&pwd.as_bytes(), &salt) {
        Ok(i) => {
            hash = i;
        }
        Err(error) => {
            println!("Error: {:?}", error);
            return "".to_string();
        }
    }
    return hash.to_string()
}
pub fn create_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn create_login_jwt(username: String) -> String {
    create_jwt(json!({"user": username}))
}

pub fn read_login_jwt(req: HttpRequest) -> String {
    if let Some(cookie) = req.cookie("login") {
        // You can access the cookie's value using the .value() method
        let cookie_value = cookie.clone().value().to_owned();
    }
    return "".to_string()
}

pub fn create_jwt(data: serde_json::Value) -> String {
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_SECRET).unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let x = Token::new(header, data).sign_with_key(&key).unwrap();
    x.as_str().to_string()
}

pub fn verify_jwt(data: &str) -> Result<String, errors> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_SECRET).unwrap();
    let token: Token<Header, BTreeMap<String, String>, _>;
    match data.verify_with_key(&key) {
        Err(e) => {
            return Err( errors::jwt_invalid("The jwt key is invalid".to_string()))
        }
        Ok(j) => {
            token = j;
        }
    }
    let claims = token.claims();
    return Ok(claims["user"].clone());
}