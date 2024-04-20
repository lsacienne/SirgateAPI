use actix_web::{HttpResponse, Responder, web};

#[actix_web::put("/rank/update")]
pub async fn update_rank(rank: web::Json<String>) -> impl Responder {
    // Deserialize JSON to Rank struct
    let rank = rank.into_inner();

    // TODO update rank in db
    // Get UUID from db

    HttpResponse::Ok().body("Rank Updated")
}
