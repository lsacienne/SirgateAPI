use actix_web::{HttpResponse, Responder, web};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use bcrypt::verify;
use jsonwebtoken::{EncodingKey, Header};

use crate::Claims;
use crate::models::user::{DGS, User, UserAuth};

/*#[actix_web::post("/users")]
pub async fn add_user(user: web::Json<models::user>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().json(user.into_inner())
}*/


/***
    TODO :
        - login
        - register
        - Add a new user to the database
        - Get all user form the database
        - Get a user by username+email
        - Update a user by id
        - Delete a user by id
        - Add DGS to database
 */

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

    let key = EncodingKey::from_base64_secret("FFGONEXT").unwrap();
    let jwt = jsonwebtoken::encode(&header, &claims, &key);

    Ok(jwt?)
}
#[actix_web::post("/login")]
pub async fn login(user: web::Json<UserAuth>) -> impl Responder {
    // Deserialize JSON to User struct
    let user: UserAuth = user.into_inner();

    // TODO get user from db

    HttpResponse::Ok().body("Login")


}

#[actix_web::post("/register")]
pub async fn register(user: web::Json<UserAuth>) -> impl Responder {
    // Deserialize JSON to User struct
    let user: UserAuth = user.into_inner();

    let salt = generate_salt();

    let user = User {
        id: 0,
        username: user.username.clone(),
        email: user.email.clone(),
        password_salt: salt.to_string(),
        password_hash: hash_password(&user.password, &salt).unwrap().hash.unwrap().to_string()
    };

    // TODO add user to db

    let claims = Claims {
        iss: user.username.clone(),
        sub: user.email.clone(),
        iat: 0,
        exp: 0,
    };
    create_jwt(claims).unwrap()
}

#[actix_web::post("/users")]
pub async fn add_user(user: web::Json<UserAuth>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
   "ff"
}

#[actix_web::get("/users")]
pub async fn get_users() -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    " Get All Users you scumbag !"
}

#[actix_web::get("/user")]
pub async fn get_user_by_username_email(user: web::Json<UserAuth>) -> impl Responder {
    // Deserialize JSON to User struct
    let user: UserAuth = user.into_inner();

    user.username + " " + &user.email + " " + &user.password
}

#[actix_web::get("/user/{id}")]
pub async fn get_user_by_id(id: web::Path<i32>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().body(format!("Get User by id: {}", id))
}

#[actix_web::put("/user/{id}")]
pub async fn update_user(id: web::Path<i32>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().body(format!("Update User by id: {}", id))
}

#[actix_web::delete("/user/{id}")]
pub async fn delete_user(id: web::Path<i32>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().body(format!("Delete User by id: {}", id))
}

#[actix_web::post("/dgs/add")]
pub async fn add_dgs( server: web::Json<DGS>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().body(format!("Add DGS: {}", server.label))
}

#[actix_web::get("/dgs/login")]
pub async fn dgs_login(server: web::Json<DGS>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    let claims = Claims {
        iss: server.id.clone().to_string(),
        sub: server.label.clone(),
        iat: 0,
        exp: 0,
    };
    HttpResponse::Ok().body(create_jwt(claims).unwrap())
}
