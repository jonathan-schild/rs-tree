/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use actix_session::SessionMiddleware;
use actix_web::{
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::Data,
    App, HttpServer,
};
use anyhow::{Error, Result};
use setup::Config;
use sqlx::PgPool;

#[allow(unused)]
mod db;

mod services;
mod setup;
mod snp_manager;
pub mod utility;

struct AppData {
    pub db: PgPool,
}

pub async fn rs_tree_run() -> Result<(), Error> {
    let config = Config::init().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(SessionMiddleware::new(
                config.redis.clone(),
                config.secret_key.clone(),
            ))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(Data::new(AppData {
                db: config.postgres.clone(),
            }))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Result::Ok(())
}
