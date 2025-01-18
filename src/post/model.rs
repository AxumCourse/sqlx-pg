use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgExecutor};

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub images: Vec<String>,
}

pub async fn create(e: impl PgExecutor<'_>, m: &Post) -> sqlx::Result<i32> {
    let (id,): (i32,) =
        sqlx::query_as(r#"INSERT INTO posts ("content", images) VALUES ($1, $2) RETURNING id"#)
            .bind(&m.content)
            .bind(&m.images)
            .fetch_one(e)
            .await?;
    Ok(id)
}

pub async fn find(e: impl PgExecutor<'_>, id: i32) -> sqlx::Result<Option<Post>> {
    sqlx::query_as(r#"SELECT id, "content", images FROM posts WHERE id = $1"#)
        .bind(id)
        .fetch_optional(e)
        .await
}
