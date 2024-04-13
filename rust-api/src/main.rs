use std::env;
use actix_web::{App, get, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use crate::controller::database_manager::establish_connection;
use std::sync::Mutex;
use diesel::r2d2::ConnectionManager;
use r2d2_postgres::{postgres, PostgresConnectionManager};
use r2d2::PooledConnection;
use r2d2_postgres::postgres::Config;


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
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let uri = match env::var("API_Address") {
        Ok(uri) => uri,
        Err(err) => {println!("Failed to get address: {}", err); return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get APIURI"))}
    };
    println!("Launched server at {}" , uri);

} */


 /*lazy_static!{
       pub static ref GLOBAL_CONNECTION: Mutex<PgConnection> = Mutex::new(establish_connection());
     }*/
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {

        let url =  env::var("DATABASE_URL").unwrap() ;

        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to Postgres DB file");
        App::new()
            .app_data(actix_web::web::Data::new(pool))
            .service(index)
            .service(view::client::login)
            .service(view::client::register)
            .service(view::client::get_users)
            .service(view::client::get_user_by_username_email)
            .service(view::client::add_dgs)
            .service(view::client::dgs_login)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
