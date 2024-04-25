use std::fmt::format;
use diesel::PgConnection;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use diesel::r2d2::ConnectionManager;

pub mod view{
    pub mod client;
    pub mod achievement;
    pub mod ranking;
    pub mod dgs;
}
pub mod models{
    pub mod client;
    pub mod achievement;
    pub mod friends;
    pub mod dgs;
    pub mod ranking;
}
pub mod controller{
    pub mod achievement;
    pub mod database_manager;
    pub mod dgs;
    pub mod client;
    pub mod ranking;
    pub mod friends;
}

mod schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

impl Claims {
    pub fn extract_uuid(self) -> Result<uuid::Uuid, uuid::Error> {
        uuid::Uuid::parse_str(&self.iss)
    }
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn handle_jwt_token(req: actix_web::HttpRequest) -> Result<Claims, actix_web::Error> {
    dotenv::dotenv().ok();
    let secret = match dotenv::var("JWT_SECRET") {
        Ok (secret) => secret,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("JWT secret phrase is not set"))
    };

    let token = req.headers().get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Authorization header is missing"))?
        .to_str()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid Authorization header"))?;
    
    let decoding_key = match DecodingKey::from_base64_secret(&secret) {
    Ok(key) => key,
    Err(_) => return Err(actix_web::error::ErrorInternalServerError("Invalid base64 secret")),
};

    let token = token.replace("Bearer ", "");
    jsonwebtoken::decode::<Claims>(&token, &decoding_key, &Validation::default())
    .map(|data| data.claims)
    .map_err(|err| {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => actix_web::error::ErrorUnauthorized("Invalid token"),
            jsonwebtoken::errors::ErrorKind::InvalidSignature => actix_web::error::ErrorUnauthorized("Invalid signature"),
            // print the error to the console
            _ => {
                actix_web::error::ErrorInternalServerError(format!("JWT validation failed: {}", err))
            },
        }
    })
}