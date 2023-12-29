use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
async fn create(
    _session: Session,
    data: Json<LoginData>,
    db: Data<DatabaseConnection>,
) -> impl Responder {
    let user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        user_name: ActiveValue::Set(data.user_name.clone()),
        password_hash: ActiveValue::Set(Some(data.password.clone())),
    };

    user.insert(db.as_ref()).await.unwrap();

    HttpResponse::Ok()
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
