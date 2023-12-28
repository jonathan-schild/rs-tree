use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout);
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    user_name: String,
    password: String,
}

#[post("/login")]
async fn login(_session: Session, _data: Json<LoginData>, _db: Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(_session: Session, _db: Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}
