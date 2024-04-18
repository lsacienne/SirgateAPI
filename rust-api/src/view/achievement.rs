use actix_web::{HttpResponse, Responder, web};

#[actix_web::post("/achievement/add")]
pub async fn add_achievement(achievement: web::Json<String>) -> impl Responder {
    // Deserialize JSON to Achievement struct
    let Achievement = achievement.into_inner();

    // TODO add achievement to db
    // Get UUID from db

    HttpResponse::Ok().body("Achievement Added")
}

#[actix_web::get("/achievement/getall")]
pub async fn get_all_achievements(client: web::Json<String>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    " Get All Achievements you scumbag !"
}
