use std::hash;
use actix_web::{error, HttpResponse, Responder, web};


use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use jsonwebtoken::{EncodingKey, Header};

use r2d2_postgres::{postgres, PostgresConnectionManager};
use r2d2_postgres::postgres::fallible_iterator::FallibleIterator;
//use crate::GLOBAL_CONNECTION;
use crate::Claims;
use crate::controller::client::{add_user, get_user_by_email, get_user_by_username};
use crate::models::client::{DGS, Client, ClientAuth, InsertableClient};
use crate::DbPool;


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
fn create_jwt(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {

    let mut header = Header::new(jsonwebtoken::Algorithm::HS256);
    header.typ = Some("JWT".to_string());

    let key = EncodingKey::from_base64_secret("FFGONEXT").unwrap();
    let jwt = jsonwebtoken::encode(&header, &claims, &key);

    Ok(jwt?)
}

#[actix_web::post("/login")]
pub async fn login(pool: web::Data<DbPool>, user: web::Json<ClientAuth>) -> actix_web::Result<impl Responder>  {
    // Deserialize JSON to User struct
    let _user: ClientAuth = user.into_inner();

    if _user.password.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Password is empty"));
    }

    if _user.email.is_empty() && _user.username.is_empty() {
         return Ok(HttpResponse::BadRequest().body("Email or username is empty"));
    }
    let client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let mut user: Client;


        if (!_user.username.is_empty()) {
            user = get_user_by_username(&mut *conn, &*_user.username);
        } else {
            user = get_user_by_email(&mut *conn, &*_user.email);
        }
        user
    }).await?;

    let salt = &mut SaltString::from_b64(&*client.salt).unwrap();
    let hashed_password = hash_password(&*_user.password, salt).unwrap();

    let argon2 = Argon2::default();

    if hashed_password.hash.unwrap().to_string() == client.password {
        let claims = Claims {
            iss: client.id.clone().to_string(),
            sub: client.email.clone(),
            iat: 0,
            exp: 0,
        };
         Ok(HttpResponse::Ok().body(create_jwt(claims).unwrap()))
    } else {
         Ok(HttpResponse::BadRequest().body("Invalid password"))
    }
}

#[actix_web::post("/register")]
pub async fn register(pool: web::Data<DbPool>, user: web::Json<ClientAuth>) -> actix_web::Result<impl Responder> {
    let user: ClientAuth = user.into_inner();


    let client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.


        let salt = generate_salt();
        let hash = hash_password(&*user.password, &salt).unwrap();

        let mut conn = pool.get().expect("couldn't get db connection from pool");

        add_user(&mut *conn, &*user.username, &*user.email, &*hash.hash.unwrap().to_string(), salt.as_ref())
    })
        .await?;


    let claims = Claims {
        iss: client.id.clone().to_string(),
        sub: client.email.clone().to_string(),
        iat: 0,
        exp: 0,
    };
    Ok(HttpResponse::Ok().body(create_jwt(claims).unwrap()))
}

#[actix_web::get("/users")]
pub async fn get_users() -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    " Get All Users you scumbag !"
}

#[actix_web::get("/singleuser")]
pub async fn get_user_by_username_email(user: web::Json<ClientAuth>) -> impl Responder {
    // Deserialize JSON to User struct
    let user: ClientAuth = user.into_inner();

    user.username.to_owned() + " " + &user.email + " " + &user.password
}

#[actix_web::post("/dgs/add")]
pub async fn add_dgs(server: web::Json<DGS>) -> impl Responder {

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
