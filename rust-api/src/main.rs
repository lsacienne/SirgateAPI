use bcrypt::{verify };
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString
    },
    Argon2
};
use jsonwebtoken::{EncodingKey, Header};
use actix_web::{web, HttpResponse, Responder, get, HttpServer, App};

mod view{
    pub mod user;
    pub mod achievement;
    pub mod ranking;
}

mod models{
    pub mod user;
    pub mod achievement;
    pub mod ranking;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

fn hash_password<'a>(password: &'a str, salt: &'a SaltString) -> Result<PasswordHash<'a>, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let salt_clone = salt.as_salt().to_owned();

    let password_hash = argon2.hash_password(password.as_bytes(), *&salt_clone)?;

    Ok(password_hash)
}
fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn verify_password(password: &str, salt: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    let salted_password = format!("{}{}", password, salt);
    verify(&salted_password, hash)
}



fn create_jwt(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    
    let mut header = Header::new(jsonwebtoken::Algorithm::HS256);
    header.typ = Some("JWT".to_string());

    //let key = EncodingKey::from_rsa_pem(include_bytes!("private_key.pem"))?;
    let key = EncodingKey::from_base64_secret("FFGONEXT").unwrap();
    let jwt = jsonwebtoken::encode(&header, &claims, &key);

    Ok(jwt?)
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

/*fn main() {
    // Hash a password
    let salt = generate_salt();
    let password_hash = hash_password("aled", &salt).unwrap();

    // pirnt the hashed password
    println!("Hashed password: {}", password_hash.hash.unwrap());
    let claims = Claims {
        iss: "FFGONEXT".to_string(),
        sub: "FFGONEXT".to_string(),
        iat: 0,
        exp: 0,
    };



    let jwt = match create_jwt(claims) {
        Ok(jwt) => jwt,
        Err(err) => {
            eprintln!("Failed to create JWT: {}", err);
            return;
        }
    };
    println!("{}", jwt);
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
