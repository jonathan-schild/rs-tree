/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct LinkTree {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub short_url: String,
    pub named_url: Option<String>,
    pub root: bool,
    pub tree: bool,
    pub redir_link: Option<String>,
    pub g_id: i32,
}

#[derive(Debug, FromRow)]
pub struct LinkEntry {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub redir_link: Option<String>,
    pub g_id: i32,
}

#[derive(Debug, FromRow)]
pub struct LinkTreeEntry {
    pub t_id: i32,
    pub e_id: i32,
}
