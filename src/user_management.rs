use actix_session::Session;
use actix_web::{
    post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use pwdpbkdf2::hash_password;
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create).service(login).service(logout);
}

const _MAX_PASSWD_LEN: usize = 64;
const _MAX_USER_LEN: usize = 64;

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    user_name: String,
    password: String,
}

#[post("/create")]
async fn create(_session: Session, data: Json<LoginData>) -> impl Responder {
    let hash = hash_password(&data.password);

    HttpResponse::Ok().body(hash)
}

#[post("/login")]
async fn login(_session: Session, _data: Json<LoginData>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(_session: Session) -> impl Responder {
    HttpResponse::Ok()
}
