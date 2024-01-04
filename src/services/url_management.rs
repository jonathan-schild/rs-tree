/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use actix_session::Session;
use actix_web::{
    delete, get, post,
    web::{Path, ServiceConfig},
    HttpResponse, Responder,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resolve)
        .service(create)
        .service(create_anon)
        .service(delete);
}

#[get("/resolve/{short}")]
async fn resolve(_id: Path<String>, _session: Session) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/create/{short}")]
async fn create(_id: Path<String>, _session: Session) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/create")]
async fn create_anon(_session: Session) -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/delete/{short}")]
async fn delete(_id: Path<String>, _session: Session) -> impl Responder {
    HttpResponse::Ok()
}
