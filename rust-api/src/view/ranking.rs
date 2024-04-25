use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::{handle_jwt_token, models::ranking::RankReq, DbPool};

#[actix_web::put("/rank/update")]
pub async fn update_rank(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    rank: web::Json<RankReq>
) -> actix_web::Result<impl Responder> {
    // Deserialize JSON to Rank struct
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
        return Err(actix_web::error::ErrorUnauthorized("Only DGS can update rank".to_string()));
    }

    let client_updated = web::block(move || {
        let rank_req = rank.into_inner();

        let mut conn = pool_clone
            .get()
            .expect("couldn't get db connection from pool");

        crate::controller::ranking::update_rank_by_name(
            &mut conn,
            &rank_req.username,
            &rank_req.rank_name
        )
    })
    .await?;

    // TODO update rank in db
    // Get UUID from db

    Ok(HttpResponse::Ok().body(format!("Rank Updated client {}", client_updated.username)))
}
