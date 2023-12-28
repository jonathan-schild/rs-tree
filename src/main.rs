use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web::Data, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod url_management;
mod user_management;

fn get_secret_key() -> Key {
    Key::generate()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = get_secret_key();
    let redis_connection_string = "redis://127.0.0.1:6379";
    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .expect("Cannot connect to Redis!");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rs-tree:rs-tree@localhost/rs-tree")
        .await
        .expect("Cannot connect to Database!");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .configure(url_management::config)
            .configure(user_management::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
