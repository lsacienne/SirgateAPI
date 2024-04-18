use crate::models::client::ClientAuth;

#[actix_web::get("/friends/getall")]
pub async fn get_all_friends(client_username: web::Json<String>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.

    // Todo get all friends from db for client
    " Get All Friends you scumbag !"
}

#[actix_web::post("/friends/add")]
pub async fn add_friend(body : web::Json<FriendAuth> ) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.

    // Todo add friend to db
    ""
}