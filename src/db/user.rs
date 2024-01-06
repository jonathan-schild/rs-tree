/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use anyhow::anyhow;
use sqlx::error::DatabaseError;
use sqlx::Error;
use sqlx::{any, prelude::FromRow, query, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub user_name: String,
    pub password_hash: Option<String>,
}

impl User {
    pub async fn count(db: &PgPool) -> anyhow::Result<i64> {
        let result = query!(r#"select count(*) from "user";"#r)
            .fetch_one(db)
            .await?;
        if let Some(r) = result.count {
            Ok(r)
        } else {
            Err(anyhow!("cannot determine number of users"))
        }
    }

    pub async fn insert(
        db: &PgPool,
        uuid: Uuid,
        user_name: &str,
        password_hash: Option<&str>,
    ) -> Result<i32, Error> {
        let id = sqlx::query!(
            r#"
insert into "user" (uuid, user_name, password_hash)
    values ($1, $2, $3) returning (id)"#r,
            uuid,
            user_name,
            password_hash
        )
        .fetch_one(db)
        .await?
        .id;

        Ok(id)
    }

    pub async fn select_by_user_name(db: &PgPool, user_name: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            Self,
            r#"select * from "user" where user_name = $1"#r,
            user_name
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn select(db: &PgPool, id: i32) -> Result<Self, Error> {
        let user = sqlx::query_as!(Self, r#"select * from "user" where id = $1"#r, id)
            .fetch_one(db)
            .await?;

        Ok(user)
    }
}

#[derive(Debug, FromRow)]
pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub group_name: String,
    pub root: bool,
}

impl Group {
    pub async fn count(db: &PgPool) -> anyhow::Result<i64> {
        let result = query!(r#"select count(*) from "group";"#r)
            .fetch_one(db)
            .await?;
        if let Some(r) = result.count {
            Ok(r)
        } else {
            Err(anyhow!("cannot determine number of groups"))
        }
    }

    pub async fn insert(
        db: &PgPool,
        uuid: Uuid,
        group_name: &str,
        root: bool,
    ) -> Result<i32, Error> {
        let id = sqlx::query!(
            r#"
insert into "group" (uuid, group_name, root)
    values ($1, $2, $3) returning (id)"#r,
            uuid,
            group_name,
            root
        )
        .fetch_one(db)
        .await?
        .id;

        Ok(id)
    }

    pub async fn select_by_group_name(db: &PgPool, group_name: &str) -> Result<Self, Error> {
        let group = sqlx::query_as!(
            Self,
            r#"select * from "group" where group_name = $1"#r,
            group_name
        )
        .fetch_one(db)
        .await?;

        Ok(group)
    }
}

#[derive(Debug, FromRow)]
pub struct UserGroups {
    pub u_id: i32,
    pub g_id: i32,
}

impl UserGroups {
    pub async fn insert(db: &PgPool, u_id: i32, g_id: i32) -> Result<(), Error> {
        let id = sqlx::query!(r#"insert into "user_groups" values ($1, $2)"#r, u_id, g_id,)
            .execute(db)
            .await?;

        Ok(())
    }
}
