use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_salt: String,
    pub password_hash: String
}




#[post("/users")]
pub async fn add_user(user: web::Json<AddUser>) -> impl Responder {
    // Here you can add the user to the database.
    // For now, let's just return the user data as JSON.
    HttpResponse::Ok().json(user.into_inner())
}
