/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use actix_session::Session;
use anyhow::Error;
use dotenv::var;
use log::info;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db::user::User, utility::hash_password};

const USER_NAME: &str = "user";
const LOGGED_IN: &str = "login";
const _IS_ROOT: &str = "root";
const _GROUPS: &str = "groups";
const IS_ADMIN: &str = "admin";

pub enum AuthorisationType {
    Auth,
}

pub async fn is_authorised(_at: AuthorisationType, _session: &Session, _db: &PgPool) -> bool {
    true
}

pub async fn create_admin_user(db: &PgPool) -> Result<(), Error> {
    if User::count(db).await? == 0 {
        User::insert(
            db,
            Uuid::nil(),
            "admin",
            &hash_password(&var("ADMIN").expect("cannot create admin user")),
        )
        .await?;
        info!("admin user created!");
    }
    Ok(())
}

pub async fn login(uid: i32, session: &Session, db: &PgPool) -> anyhow::Result<()> {
    // TODO set session variables at login
    let user = User::select(db, uid).await.unwrap(); // FIXME unwrap
    session.insert(USER_NAME, &user.user_name).unwrap(); // FIXME unwrap
    session.insert(LOGGED_IN, true).unwrap(); // FIXME unwrap
    session
        .insert(
            IS_ADMIN,
            user.user_name == "admin" && user.uuid == Uuid::nil(),
        )
        .unwrap(); // FIXME unwrap
    info!("login user: {} {} {}", user.id, user.user_name, user.uuid);
    Result::Ok(())
}
