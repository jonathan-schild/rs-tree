// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::scope,
    App, HttpServer,
};
use base64::{engine::general_purpose, Engine};
use dotenv::var;
use log::{error, info, warn};

mod db;
mod url_management;
mod user_management;
mod utility;

fn read_secrete_key() -> Key {
    if let Ok(base64_key) = var("COOKIE_KEY") {
        if let Ok(key_slice) = general_purpose::STANDARD.decode(base64_key) {
            if let Ok(k) = Key::try_from(key_slice.as_slice()) {
                info!("key loaded");
                k
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("start logging");

    let redis_connection_string = var("REDIS_URL").expect("accessing environment failed");
    info!("redis: {}", redis_connection_string);

    let postgres_connection_string = var("DATABASE_URL").expect("accessing environment failed");
    info!("postgres: {}", postgres_connection_string);

    let port = var("SERVER_PORT").unwrap_or("8080".to_owned());
    info!("server port: {}", port);
    let port = port.parse().expect("parsing port failed");

    let secret_key = read_secrete_key();

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .expect("Cannot connect to Redis!");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(scope("/user").configure(user_management::config))
            .configure(url_management::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
