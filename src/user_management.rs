use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use base64::{engine::general_purpose, Engine};
use log::info;
use pwdpbkdf2::{hash_password, verify_password};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::user::User, AppData};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create).service(login).service(logout);
}

// TODO checks
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
    app_data: Data<AppData>,
) -> impl Responder {
    // TODO check authorisation

    if let Ok(_) = sqlx::query!(
        r#"insert into "user" (uuid, user_name, password_hash) values ($1, $2, $3) "#r,
        Uuid::new_v4(),
        data.user_name,
        hash_password(&data.password)
    )
    .execute(&app_data.db)
    .await
    {
        HttpResponse::Ok().body(format!("Created User: {}", data.user_name))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[post("/login")]
async fn login(session: Session, data: Json<LoginData>, app_data: Data<AppData>) -> impl Responder {
    if let Ok(user) = sqlx::query_as!(
        User,
        r#"select * from "user" where user_name = $1"#r,
        data.user_name
    )
    .fetch_one(&app_data.db)
    .await
    {
        match user.password_hash {
            Some(hash) => {
                if verify_password(&data.password, &hash) {
                    session.insert("login", true).unwrap();
                    info!("login: {}", user.user_name);
                    HttpResponse::Ok()
                } else {
                    info!("wrong password: {}", user.user_name);
                    HttpResponse::Unauthorized()
                }
            }
            None => {
                info!("account locked: {}", user.user_name);
                HttpResponse::Unauthorized()
            }
        }
    } else {
        info!(
            "invalid user name: {}",
            general_purpose::STANDARD.encode(data.user_name.as_bytes())
        );
        if hash_password(&data.password) == "DasKannNichtSein!NoQuickExit" {
            // https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html#authentication-and-error-messages
            info!("lucky!")
        };
        HttpResponse::Unauthorized()
    }
}

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok()
}
