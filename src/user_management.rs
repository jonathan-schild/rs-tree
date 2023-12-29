use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout);
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    user_name: String,
    password: String,
}

#[post("/login")]
async fn login(
    _session: Session,
    _data: Json<LoginData>,
    _db: Data<DatabaseConnection>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(_session: Session, _db: Data<DatabaseConnection>) -> impl Responder {
    HttpResponse::Ok()
}
