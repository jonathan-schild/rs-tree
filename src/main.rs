use actix_web::{App, HttpServer};

mod url_management;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(url_management::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
