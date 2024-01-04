/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use actix_session::Session;
use actix_web::{
    delete, get, post,
    web::{Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::vec::from_elem;
use uuid::Uuid;

use crate::{db::link_tree::LinkTree, AppData};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resolve)
        .service(create)
        .service(create_anon)
        .service(delete);
}

const SHORT_LINK_CHARACTER: [char; 38] = [
    'a', 'b', 'c', 'd', 'e', 'g', 'h', 'k', 'o', 'p', 'r', 's', 't', 'w', 'x', 'z', 'A', 'C', 'D',
    'E', 'G', 'H', 'L', 'P', 'Q', 'R', 'S', 'T', 'U', 'W', 'X', 'Y', 'Z', '3', '4', '5', '6', '7',
];

// const LINK_CHARACTER: [char; 62] = [
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
//     'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
//     '5', '6', '7', '8', '9',
// ];

fn generate_short() -> String {
    let n = 7;
    let mut rng = from_elem(0, n);

    OsRng.fill_bytes(&mut rng);

    let mut s = rng
        .iter()
        .map(|r| SHORT_LINK_CHARACTER[(r % SHORT_LINK_CHARACTER.len() as u8) as usize])
        .fold(String::new(), |mut s, c| {
            s.push(c);
            s
        });
    s.insert(3, '-');
    s
}

#[derive(Debug, Serialize, Deserialize)]
enum LinkTreeResult {
    Redir {
        uuid: Uuid,
        name: String,
        target: String,
        root: bool,
    },
}

#[get("/resolve/{short}")]
async fn resolve(
    short: Path<String>,
    _session: Session,
    app_data: Data<AppData>,
) -> impl Responder {
    let _ = LinkTree::select_by_link(&short, &app_data.db).await;

    HttpResponse::Ok().body((0..1000).fold(String::new(), |mut s, _| {
        s.push_str(&generate_short());
        s.push('\n');
        s
    }))
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
