use std::env;
use actix_web::{App, get, HttpServer, Responder};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use rust_api::controller;

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


 /*lazy_static!{
       pub static ref GLOBAL_CONNECTION: Mutex<PgConnection> = Mutex::new(establish_connection());
     }*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    /*let uri = match env::var("API_Address") {
        Ok(uri) => uri,
        Err(err) => {println!("Failed to get address: {}", err); return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get APIURI"))}
    };*/

    let con = controller::database_manager::establish_redis_connection();
    match &con {
        Ok(_) => println!("Connected to Redis"),
        Err(err) => {println!("Failed to connect to Redis: {}", err) }
    };
    controller::dgs::setup_dgs_map(con.unwrap());

    let url =  env::var("DATABASE_URL").unwrap() ;
    let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to Postgres DB file");

    println!("Launched server ...");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(index)
            .service(rust_api::view::client::login)
            .service(rust_api::view::client::register)
            .service(rust_api::view::client::get_users)
            .service(rust_api::view::client::get_user_by_username_email)
            .service(rust_api::view::client::add_dgs)
            .service(rust_api::view::client::dgs_login)
            .service(rust_api::view::dgs::find_dgs)
            .service(rust_api::view::dgs::register_dgs)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
