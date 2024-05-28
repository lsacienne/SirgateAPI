
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};


use crate::controller::database_manager::establish_redis_connection;
//use crate::GLOBAL_CONNECTION;
use crate::{handle_jwt_token, Claims};
use crate::controller::client::{add_user, cache_client, get_user_by_email, get_user_by_username, is_client_cached};
use crate::models::client::{Client, ClientAuth};
use crate::DbPool;

pub fn hash_password<'a>(password: &'a str, salt: &'a SaltString) -> Result<PasswordHash<'a>, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let salt_clone = salt.as_salt().to_owned();

    let password_hash = argon2.hash_password(password.as_bytes(), *&salt_clone)?;

    Ok(password_hash)
}

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn create_jwt(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv::dotenv().ok();
    let secret = match dotenv::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err(jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey.into())
    };

    let mut header = Header::new(jsonwebtoken::Algorithm::HS256);
    header.typ = Some("JWT".to_string());

    let key = EncodingKey::from_base64_secret(&secret).unwrap();
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
    let pool_clone = pool.clone();
    let client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let user: Client;


        if !_user.username.is_empty() {
            user = get_user_by_username(&mut *conn, &*_user.username);
        } else {
            user = get_user_by_email(&mut *conn, &*_user.email);
        }
        user
    }).await?;

    let salt = &mut SaltString::from_b64(&*client.salt).unwrap();
    let hashed_password = hash_password(&*_user.password, salt).unwrap();

    let _argon2 = Argon2::default();

    if hashed_password.hash.unwrap().to_string() == client.password {
        let now = Utc::now().timestamp();
        let exp = now + 3600;
        let claims = Claims {
            iss: client.id.clone().to_string(),
            sub: client.email.clone().to_string(),
            iat:now,
            exp: i64::MAX,
        };
        
        let con = match establish_redis_connection() {
            Ok(con) => con,
            Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
        };

        if !is_client_cached(con, client.id) {
            web::block(move || {
                let redis_con = establish_redis_connection()
                    .expect("couldn't get redis connection");

                let mut conn = pool_clone
                    .get()
                    .expect("couldn't get db connection from pool");
                cache_client(&mut conn, redis_con, client.id);
            }).await?;
            
        }
         Ok(HttpResponse::Ok().body(create_jwt(claims).unwrap()))
    } else {
         Ok(HttpResponse::BadRequest().body("Invalid password"))
    }
}

#[actix_web::post("/register")]
pub async fn register(pool: web::Data<DbPool>, user: web::Json<ClientAuth>) -> actix_web::Result<impl Responder> {
    let user: ClientAuth = user.into_inner();

    let pool_clone = pool.clone();
    let client = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.


        let salt = generate_salt();
        let hash = hash_password(&*user.password, &salt).unwrap();

        let mut conn = pool.get().expect("couldn't get db connection from pool");

        add_user(&mut *conn, &*user.username, &*user.email, &*hash.hash.unwrap().to_string(), salt.as_ref())
    })
    .await?;


    let now = Utc::now().timestamp();
    let exp = now + 3600;
    let claims = Claims {
        iss: client.id.clone().to_string(),
        sub: client.email.clone().to_string(),
        iat: now,
        exp: i64::MAX,
    };

    web::block(move || {
        let redis_con = establish_redis_connection()
            .expect("couldn't get redis connection");

        let mut conn = pool_clone
            .get()
            .expect("couldn't get db connection from pool");
        cache_client(&mut conn, redis_con, client.id);
    }).await?;

    Ok(HttpResponse::Ok().body(create_jwt(claims).unwrap()))
}

#[actix_web::post("/logout")]
pub async fn logout(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: uuid::Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };
    let res = crate::controller::client::uncache_client(con, id);

    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::get("/singleuser")]
pub async fn get_user_by_username_email(req :HttpRequest, pool: web::Data<DbPool> ) ->  actix_web::Result<impl Responder> {

    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: uuid::Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let user = web::block(move || {
        let mut conn = pool
            .get()
            .expect("couldn't get db connection from pool");

        crate::controller::client::get_limited_user_by_id(&mut conn, id)
    }).await?;


    Ok(HttpResponse::Ok().json(user))

}
