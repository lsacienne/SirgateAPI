use actix_web::{web, HttpRequest, Responder};

use crate::{controller::{self, database_manager::establish_redis_connection}, models::{client::ClientAuth, friends::FriendAuth}, DbPool};

#[actix_web::get("/friends/getall")]
pub async fn get_all_friends(req: HttpRequest,pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // Here you can add the user to the database.
    let claim = match crate::handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };

    let id = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let friends = web::block(move || {
        let mut conn = pool
            .get()
            .expect("couldn't get db connection from pool");

        crate::controller::friends::get_friends(&mut conn, id)
    })
    .await?;

    let con = match establish_redis_connection() {
        Ok(con) => con,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let friends = controller::friends::check_friendship(con, friends);

    Ok(actix_web::HttpResponse::Ok().json(friends))
}

#[actix_web::post("/friends/add")]
pub async fn add_friend(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    friend_username: web::Json<String>
) -> actix_web::Result<impl Responder> {
    // Here you can add the user to the database.
    let claim = match crate::handle_jwt_token(req) {
        Ok(claim) => claim,
        Err(err) => return Err(err)
    };

    let id = match claim.extract_uuid() {
        Ok(id) => id,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err))
    };

    let friend = web::block(move || {
        let mut conn = pool
            .get()
            .expect("couldn't get db connection from pool");

        let friend_username = friend_username.into_inner();

        let friend = crate::controller::client::get_user_by_username(&mut conn, &friend_username);

        crate::controller::friends::add_friend(&mut conn, id, friend.id)
    })
    .await?;

    Ok(actix_web::HttpResponse::Ok().body(format!("Friend added: {}", friend.client2_id)))
}