use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::{handle_jwt_token, DbPool};

#[actix_web::post("/achievement/add")]
pub async fn add_achievement(_pool: web::Data<DbPool>, achievement: web::Json<String>) -> impl Responder {
    // Deserialize JSON to Achievement struct
    let _Achievement = achievement.into_inner();

    // TODO add achievement to db
    // Get UUID from db

    HttpResponse::Ok().body("Achievement Added")
}

#[actix_web::get("/achievement/getall")]
pub async fn get_all_achievements(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // Here you can add the user to the database.
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let achievements = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::controller::achievement::get_all_achievements(&mut conn, id)
    }).await?;

    Ok(HttpResponse::Ok().json(achievements))
}
