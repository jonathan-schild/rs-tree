use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, App, HttpServer};

mod url_management;

fn get_secret_key() -> Key {
    Key::generate()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = get_secret_key();
    let redis_connection_string = "redis://127.0.0.1:6379";
    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .configure(url_management::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
