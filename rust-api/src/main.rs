use std::env;
use actix_web::{App, get, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod view{
    pub mod client;
    pub mod user;
    pub mod achievement;
    pub mod ranking;
}
mod models{
    pub mod client;
    pub mod user;
    pub mod achievement;
    pub mod ranking;
    pub mod friends;
}
mod controller{
    pub mod database_manager;
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

} */
 #[actix_web::main]
 async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    /*let uri = match env::var("API_Address") {
        Ok(uri) => uri,
        Err(err) => {println!("Failed to get address: {}", err); return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get APIURI"))}
    };*/
    println!("Launched server ");
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
        .bind("localhost:8080")?
        .run()
        .await
}
