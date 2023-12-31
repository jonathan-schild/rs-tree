/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Acl {
    pub e_id: i32,
    pub g_id: i32,
    pub read: bool,
    pub write: bool,
}
