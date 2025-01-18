use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{ArcAppState, Error, Result};

use super::model;

#[derive(Deserialize)]
pub struct CreateForm {
    pub title: String,
    pub author: String,
}
pub async fn create(
    State(state): State<ArcAppState>,
    Json(frm): Json<CreateForm>,
) -> Result<Json<i32>> {
    let id = model::create(&state.pool, &model::Topic::new(frm.title, frm.author)).await?;
    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct ListForm {
    pub title: Option<String>,
    pub author: Option<String>,
}
pub async fn list(
    State(state): State<ArcAppState>,
    Query(frm): Query<ListForm>,
) -> Result<Json<Vec<model::Topic>>> {
    let ls = model::list(
        &state.pool,
        &model::TopicFilter {
            title: frm.title,
            author: frm.author,
        },
    )
    .await?;
    Ok(Json(ls))
}

pub async fn meta(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<model::TopicMeta>>> {
    let meta = model::get_meta(&state.pool, id).await?;
    Ok(Json(meta))
}

pub async fn author(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<String>>> {
    let author = model::get_author(&state.pool, id).await?;
    Ok(Json(author))
}

pub async fn views(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<i64>>> {
    let views = model::get_views(&state.pool, id).await?;
    Ok(Json(views))
}

pub async fn increment_views(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<i64>> {
    let mut tx = state.pool.begin().await?;
    let meta = match model::get_meta(&mut *tx, id).await {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::new("不存在的记录")),
        },
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };
    let views = match model::increment_views(&mut *tx, id, meta).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };
    tx.commit().await?;
    Ok(Json(views))
}
