
use std::time::Duration;

use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Local;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

fn hash_password(password: &str, salt: &str) -> Result<String, bcrypt::BcryptError> {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt.as_ref())?.to_string();

    Ok(password_hash)
}

fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

fn verify_password(password: &str, salt: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    let salted_password = format!("{}{}", password, salt);
    verify(&salted_password, hash)
}

fn initialize_claim(client_id: i32 ) -> Claims {
    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    Claims {
        iss: client_id.to_string(),
        sub: Uuid::new_v4().to_string(),
        iat,
        exp,
    }
}

fn create_jwt(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    
    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.typ = Some("JWT".to_string());

    let key = EncodingKey::from_rsa_pem(include_bytes!("private_key.pem"))?;
    let jwt = encode(&header, &claims, &key);

    Ok(jwt?)
}

fn main() {
    // Hash a password
    let salt = "random_salt".to_string();
    let password_hash = hash_password("password", &salt).unwrap();
    println!("Password Hash: {}", password_hash);

    // Verify a password
    let is_valid = verify_password("password", &salt, &password_hash).unwrap();
    println!("Password is valid: {}", is_valid);

    let token = create_jwt(claims).unwrap();
    println!("JWT: {}", token);
}
