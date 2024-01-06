/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use anyhow::{Error, Result};
use base64::{engine::general_purpose, Engine};
use dotenv::var;
use log::{debug, error, info, warn};
use sqlx::{migrate, PgPool};
use uuid::Uuid;

use crate::{
    db::user::{Group, User, UserGroups},
    utility::{build_info, hash_password},
};

async fn create_admin_user(db: &PgPool) -> Result<(), Error> {
    if User::count(db).await? == 0 {
        let u_id = User::insert(
            db,
            Uuid::nil(),
            "admin",
            hash_password(
                &var("ADMIN")
                    .expect("env variable `ADMIN` must provide the 'admin' initial password"),
            )
            .as_deref(),
        )
        .await?;
        info!("admin user created!");

        if Group::count(db).await? == 0 {
            let g_id = Group::insert(db, Uuid::nil(), "root", true).await?;
            info!("root group created!");

            UserGroups::insert(db, u_id, g_id).await?;
        }
    }

    Ok(())
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

pub struct Config {
    pub postgres: PgPool,
    pub redis: RedisSessionStore,
    pub port: u16,
    pub api_url: String,
    pub secret_key: Key,
}

impl Config {
    pub async fn init() -> Self {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        info!("{}", build_info());

        let redis_connection_string = var("REDIS_URL").expect("env variable `REDIS` must be set");
        info!("redis: {}", redis_connection_string);

        let postgres_connection_string =
            var("DATABASE_URL").expect("env variable `DATABASE_URL` must be set");
        info!("postgres: {}", postgres_connection_string);

        let port = if let Ok(port) = var("SERVER_PORT") {
            port
        } else {
            warn!("env variable `SERVER_PORT` is not set. default to `8080`");
            "8080".to_owned()
        };
        info!("server port: {}", port);
        let port = port
            .parse()
            .unwrap_or_else(|_| panic!("{port} is not a valid port"));

        let api_url = var("URL").unwrap_or_default();
        info!("api url: {}", api_url);

        let secret_key = read_secrete_key();

        let postgres = PgPool::connect(&postgres_connection_string)
            .await
            .expect("cannot connect to PostgreSQL");
        info!("connected to database");

        migrate!("./migrations")
            .run(&postgres)
            .await
            .expect("database migrations failed");
        info!("applied migrations");

        create_admin_user(&postgres)
            .await
            .expect("creation of user 'admin' failed");

        let redis = RedisSessionStore::new(redis_connection_string)
            .await
            .expect("Cannot connect to Redis!");
        info!("connected to session store");

        Config {
            postgres,
            redis,
            port,
            api_url,
            secret_key,
        }
    }
}
