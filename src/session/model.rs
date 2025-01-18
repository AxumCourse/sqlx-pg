use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: i32,
    pub token: String,
    pub data: sqlx::postgres::types::PgHstore,
}

impl Session {
    pub fn new() -> Self {
        let token = xid::new().to_string();
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn set_data(mut self, data: BTreeMap<String, String>) -> Self {
        data.into_iter().for_each(|(key, value)| {
            self.data.insert(key, Some(value));
        });
        self
    }

    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), Some(value.into()));
        self
    }

    pub fn with_data(data: BTreeMap<String, String>) -> Self {
        let s = Self::new();

        s.set_data(data)
    }
}

impl Into<BTreeMap<String, String>> for Session {
    fn into(self) -> BTreeMap<String, String> {
        self.data
            .into_iter()
            .map(|(key, val)| (key, val.unwrap_or_default()))
            .collect()
    }
}

pub async fn create(e: impl PgExecutor<'_>, m: &Session) -> sqlx::Result<String> {
    let (token,): (String,) = sqlx::query_as(
        r#"INSERT INTO sessions ("token", "data") VALUES ($1, $2) RETURNING "token""#,
    )
    .bind(&m.token)
    .bind(&m.data)
    .fetch_one(e)
    .await?;
    Ok(token)
}

pub async fn find_by_token(e: impl PgExecutor<'_>, token: &str) -> sqlx::Result<Option<Session>> {
    sqlx::query_as(r#"SELECT id, "token", "data" FROM sessions WHERE "token" = $1"#)
        .bind(token)
        .fetch_optional(e)
        .await
}

pub async fn find_by_email(e: impl PgExecutor<'_>, email: &str) -> sqlx::Result<Option<Session>> {
    sqlx::query_as(r#"SELECT id, "token", "data" FROM sessions WHERE "data" -> 'email' = $1 ORDER BY id DESC LIMIT 1"#)
        .bind(email)
        .fetch_optional(e)
        .await
}
