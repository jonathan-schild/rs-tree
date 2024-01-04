/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use base64::{engine::general_purpose, Engine};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::user::User, snp_manager::AuthorisationType, utility::verify_password, AppData};
use crate::{
    snp_manager::{self as snp, is_authorised},
    utility::hash_password,
};

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
    session: Session,
    data: Json<LoginData>,
    app_data: Data<AppData>,
) -> impl Responder {
    if is_authorised(AuthorisationType::UserManagement, &session, &app_data.db).await {
        if User::insert(
            &app_data.db,
            Uuid::new_v4(),
            &data.user_name,
            &hash_password(&data.password),
        )
        .await
        .is_ok()
        {
            HttpResponse::Ok().body(format!("Created User: {}", data.user_name))
        } else {
            HttpResponse::Forbidden().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[post("/login")]
async fn login(session: Session, data: Json<LoginData>, app_data: Data<AppData>) -> impl Responder {
    if is_authorised(AuthorisationType::Login, &session, &app_data.db).await {
        return HttpResponse::Ok();
    }

    if let Ok(user) = User::select_by_user_name(&app_data.db, &data.user_name).await {
        if let Some(hash) = user.password_hash {
            if verify_password(&data.password, &hash) {
                snp::login(user.id, &session, &app_data.db).await.unwrap();
                HttpResponse::Ok()
            } else {
                info!("wrong password: {}", user.user_name);
                HttpResponse::Unauthorized()
            }
        } else {
            info!("account locked: {}", user.user_name);
            HttpResponse::Unauthorized()
        }
    } else {
        info!(
            "invalid user name: {}",
            general_purpose::STANDARD.encode(data.user_name.as_bytes())
        );
        if hash_password(&data.password) == "DasKannNichtSein!NoQuickExit" {
            // https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html#authentication-and-error-messages
            error!("lucky!");
        };
        HttpResponse::Unauthorized()
    }
}

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok()
}
