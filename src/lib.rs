/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{scope, Data},
    App, HttpServer,
};
use anyhow::{Error, Result};
use base64::{engine::general_purpose, Engine};
use dotenv::var;
use log::{debug, error, info, warn};
use sqlx::{migrate, PgPool};

use crate::{
    services::{url_management, user_management},
    snp_manager::create_admin_user,
    utility::build_info,
};

#[allow(unused)]
mod db;
mod services;
mod snp_manager;
mod utility;

struct AppData {
    pub db: PgPool,
}

/// Read cookie key from env (see [`actix_session`]).
fn read_secrete_key() -> Key {
    if let Ok(base64_key) = var("COOKIE_KEY") {
        if let Ok(key_slice) = general_purpose::STANDARD.decode(&base64_key) {
            if let Ok(key) = Key::try_from(key_slice.as_slice()) {
                debug! {"key: {}", base64_key}
                info!("key loaded");
                key
            } else {
                let key = Key::generate();
                error!(
                    "not a valid key! using key: {}",
                    general_purpose::STANDARD.encode(key.master())
                );
                key
            }
        } else {
            let key = Key::generate();
            error!(
                "key could not be decoded! using key: {}",
                general_purpose::STANDARD.encode(key.master())
            );
            key
        }
    } else {
        let key = Key::generate();
        warn!(
            "key could not read from environment! using key: {}",
            general_purpose::STANDARD.encode(key.master())
        );
        key
    }
}

pub async fn rs_tree_run() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", build_info());

    let redis_connection_string = var("REDIS_URL").expect("accessing environment failed");
    info!("redis: {}", redis_connection_string);

    let postgres_connection_string = var("DATABASE_URL").expect("accessing environment failed");
    info!("postgres: {}", postgres_connection_string);

    let port = var("SERVER_PORT").unwrap_or("8080".to_owned());
    info!("server port: {}", port);
    let port = port.parse().expect("parsing port failed");

    let api_url = var("URL").expect("accessing environment failed");
    info!("api url: {}", api_url);

    let secret_key = read_secrete_key();

    let db = PgPool::connect(&postgres_connection_string)
        .await
        .expect("Cannot connect to PostgreSQL");
    info!("connected to database");

    create_admin_user(&db).await?;

    migrate!("./migrations")
        .run(&db)
        .await
        .expect("Cannot run migrations!");
    info!("applied migrations");

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .expect("Cannot connect to Redis!");
    info!("connected to session store");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(Data::new(AppData { db: db.clone() }))
            .service(
                scope(&api_url)
                    .service(scope("/user").configure(user_management::config))
                    .configure(url_management::config),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Result::Ok(())
}
