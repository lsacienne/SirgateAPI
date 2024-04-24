use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::{controller::achievement, handle_jwt_token, models::achievement::ClientAchievementReq, DbPool};

#[actix_web::post("/achievement/add")]
pub async fn add_achievement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    achievement: web::Json<ClientAchievementReq>,
    
) -> actix_web::Result<impl Responder> {
    let claim = match handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };
    let id: Uuid = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let pool_clone = pool.clone();
    let is_requester_dgs = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool
            .get()
            .expect("couldn't get db connection from pool");

        crate::controller::client::is_user_dgs(&mut conn, id)
    }).await?;

    if !is_requester_dgs {
        return Err(actix_web::error::ErrorUnauthorized("Only DGS can add achievements".to_string()));
    }

    let achievement = web::block(move || {
        let client_achievement_req = achievement.into_inner();

        let mut conn = pool_clone
            .get()
            .expect("couldn't get db connection from pool");

        crate::controller::achievement::add_achievement_by_name(
            &mut conn,
            &client_achievement_req.client_username,
            &client_achievement_req.achievement_name
        )
    })
    .await?;

    Ok(HttpResponse::Ok().json(achievement))
}

/// Used from a client player
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
