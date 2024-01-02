use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub user_name: String,
    pub password_hash: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub root: bool,
}

#[derive(Debug, FromRow)]
pub struct UserGroups {
    pub u_id: i32,
    pub g_id: i32,
}
