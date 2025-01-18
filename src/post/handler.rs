use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{ArcAppState, Error, Result};

use super::model;

#[derive(Deserialize)]
pub struct CreateForm {
    content: String,
    images: Vec<String>,
}
pub async fn create(
    State(state): State<ArcAppState>,
    Json(frm): Json<CreateForm>,
) -> Result<Json<i32>> {
    let id = model::create(
        &state.pool,
        &model::Post {
            content: frm.content,
            images: frm.images,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(id))
}

pub async fn find(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<model::Post>> {
    let post = match model::find(&state.pool, id).await? {
        Some(v) => v,
        None => return Err(Error::new("不存在的记录")),
    };

    Ok(Json(post))
}
