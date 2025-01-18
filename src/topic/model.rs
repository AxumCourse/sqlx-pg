use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TopicMeta {
    pub author: String,
    pub views: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub meta: sqlx::types::Json<TopicMeta>,
}

impl Topic {
    pub fn new(title: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            meta: sqlx::types::Json(TopicMeta {
                author: author.into(),
                views: 0,
            }),
            ..Default::default()
        }
    }
}

pub struct TopicFilter {
    pub title: Option<String>,
    pub author: Option<String>,
}

pub async fn create(e: impl PgExecutor<'_>, m: &Topic) -> sqlx::Result<i32> {
    let (id,): (i32,) =
        sqlx::query_as(r#"INSERT INTO topics (title, "meta") VALUES ($1, $2) RETURNING id"#)
            .bind(&m.title)
            .bind(&m.meta)
            .fetch_one(e)
            .await?;
    Ok(id)
}

pub async fn list(e: impl PgExecutor<'_>, f: &TopicFilter) -> sqlx::Result<Vec<Topic>> {
    let mut q = sqlx::QueryBuilder::new(r#"SELECT id, title, "meta" FROM topics WHERE 1=1"#);

    if let Some(v) = &f.title {
        q.push(" AND title ILIKE ").push_bind(format!("%{}%", v));
    }
    if let Some(v) = &f.author {
        q.push(r#" AND "meta" ->> 'author' ILIKE "#)
            .push_bind(format!("%{}%", v));
    }

    q.build_query_as().fetch_all(e).await
}

pub async fn get_meta(e: impl PgExecutor<'_>, id: i32) -> sqlx::Result<Option<TopicMeta>> {
    let r: Option<(sqlx::types::Json<TopicMeta>,)> =
        sqlx::query_as(r#"SELECT "meta" FROM topics WHERE id = $1"#)
            .bind(id)
            .fetch_optional(e)
            .await?;
    let meta = match r {
        Some((v,)) => Some(v.0),
        None => None,
    };

    Ok(meta)
}

pub async fn get_author(e: impl PgExecutor<'_>, id: i32) -> sqlx::Result<Option<String>> {
    let r: Option<(String,)> =
        sqlx::query_as(r#"SELECT "meta" ->> 'author' FROM topics WHERE id = $1"#)
            .bind(id)
            .fetch_optional(e)
            .await?;
    let author = match r {
        Some((v,)) => Some(v),
        None => None,
    };

    Ok(author)
}

pub async fn get_views(e: impl PgExecutor<'_>, id: i32) -> sqlx::Result<Option<i64>> {
    let r: Option<(i64,)> =
        sqlx::query_as(r#"SELECT ("meta" -> 'views')::INT8 AS views FROM topics WHERE id = $1"#)
            .bind(id)
            .fetch_optional(e)
            .await?;
    let views = match r {
        Some((v,)) => Some(v),
        None => None,
    };

    Ok(views)
}

pub async fn increment_views(
    e: impl PgExecutor<'_>,
    id: i32,
    meta: TopicMeta,
) -> sqlx::Result<i64> {
    let meta = sqlx::types::Json(TopicMeta {
        views: meta.views + 1,
        ..meta
    });
    let (views,): (i64,) = sqlx::query_as(
        r#"UPDATE topics SET "meta" = $1 WHERE id = $2 RETURNING ("meta" -> 'views')::INT8"#,
    )
    .bind(&meta)
    .bind(id)
    .fetch_one(e)
    .await?;
    Ok(views)
}
