use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

use crate::Result;

#[derive(Debug, Default, Deserialize, Serialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

pub async fn insert(p: &PgPool, m: &Category) -> Result<i32> {
    let (id,): (i32,) =
        sqlx::query_as(r#"INSERT INTO categories ("name") VALUES ($1) RETURNING id"#)
            .bind(&m.name)
            .fetch_one(p)
            .await?;
    Ok(id)
}

pub async fn update(p: &PgPool, m: &Category) -> Result<u64> {
    let aff = sqlx::query(r#"UPDATE categories SET "name" = $1 WHERE id = $2"#)
        .bind(&m.name)
        .bind(&m.id)
        .execute(p)
        .await?
        .rows_affected();
    Ok(aff)
}

pub async fn delete(p: &PgPool, id: i32) -> Result<u64> {
    let aff = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(p)
        .await?
        .rows_affected();
    Ok(aff)
}

pub async fn find(p: &PgPool, id: i32) -> Result<Option<Category>> {
    let m = sqlx::query_as(r#"SELECT id, "name" FROM categories WHERE id = $1"#)
        .bind(id)
        .fetch_optional(p)
        .await?;
    Ok(m)
}

pub async fn list(p: &PgPool) -> Result<Vec<Category>> {
    let m = sqlx::query_as(r#"SELECT id, "name" FROM categories ORDER BY id DESC LIMIT 100"#)
        .fetch_all(p)
        .await?;
    Ok(m)
}
