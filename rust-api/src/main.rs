use actix_web::{App, get, HttpServer, Responder};
use argon2::password_hash::PasswordHasher;
use serde::{Deserialize, Serialize};

mod view{
    pub mod client;
    pub mod achievement;
    pub mod ranking;
}
mod models{
    pub mod client;
    pub mod achievement;
    pub mod dgs;
    pub mod ranking;
    pub mod friends;
}
mod controller{
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


#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

/*fn main() {
    // Hash a password
    /*
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
     */
}
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(view::user::add_user)
            .service(view::user::get_users)
            .service(view::user::get_user_by_username_email)
            .service(view::user::login)
            .service(view::user::register)
            .service(view::user::update_user)
            .service(view::user::delete_user)
            .service(view::user::add_dgs)
            .service(view::user::get_user_by_id)
            .service(view::user::dgs_login)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
