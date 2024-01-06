/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use sqlx::{query_as, Error, FromRow, PgPool};
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

impl LinkTree {
    pub async fn insert(
        db: &PgPool,
        name: &str,
        short_url: &str,
        target: Option<&str>,
        g_id: i32,
    ) -> Result<i32, Error> {
        let id = sqlx::query!(
            r#"
insert into "link_tree"
    (uuid, name, short_url, redir_link, g_id)
    VALUES ($1, $2, $3, $4, $5) returning (id)
"#r,
            Uuid::new_v4(),
            name,
            short_url,
            target,
            g_id
        )
        .fetch_one(db)
        .await?
        .id;

        Ok(id)
    }

    #[must_use]
    pub async fn select_by_link(link: &str, db: &PgPool) -> Option<Self> {
        if let Ok(t) = query_as!(
            Self,
            r#"
select * from "link_tree"
    where $1 = short_url OR $1 = named_url "#r,
            link
        )
        .fetch_one(db)
        .await
        {
            Some(t)
        } else {
            None
        }
    }
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
