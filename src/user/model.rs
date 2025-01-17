/*
CREATE TABLE IF NOT EXISTS "users"(
    "id" SERIAL PRIMARY KEY,
    "username" VARCHAR(50) NOT NULL UNIQUE
); */

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgExecutor, PgTransaction};

use crate::{Error, Result};

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}

pub async fn exists(e: impl PgExecutor<'_>, username: &str, id: Option<i32>) -> sqlx::Result<bool> {
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM users WHERE username = ");
    q.push_bind(username);
    if let Some(v) = id {
        q.push(" AND id <>");
        q.push_bind(v);
    }

    let (c,): (i64,) = q.build_query_as().fetch_one(e).await?;
    Ok(c > 0)
}

pub async fn insert(e: impl PgExecutor<'_>, m: &User) -> sqlx::Result<i32> {
    let (id,): (i32,) =
        sqlx::query_as(r#"INSERT INTO users ("username") VALUES ($1) RETURNING id"#)
            .bind(&m.username)
            .fetch_one(e)
            .await?;
    Ok(id)
}

pub async fn update(e: impl PgExecutor<'_>, m: &User) -> sqlx::Result<u64> {
    let aff = sqlx::query(r#"UPDATE users SET "username" = $1 WHERE id = $2"#)
        .bind(&m.username)
        .bind(&m.id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

pub async fn register(tx: &mut PgTransaction<'_>, m: &User) -> Result<i32> {
    let user_exists = exists(&mut **tx, &m.username, None).await?;
    if user_exists {
        return Err(Error::new("用户已存在"));
    }

    let id = insert(&mut **tx, m).await?;
    Ok(id)
}
// pub async fn register_wrong(e: impl PgExecutor<'_>, m: &User) -> Result<i32> {
//     let user_exists = exists(e, &m.username, None).await?;
//     if user_exists {
//         return Err(Error::new("用户已存在"));
//     }

//     let id = insert(e, m).await?;
//     Ok(id)
// }
