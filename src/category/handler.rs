use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{ArcAppState, Result};

use super::model;

#[derive(Deserialize)]
pub struct CreateForm {
    pub name: String,
}

pub async fn create(
    State(state): State<ArcAppState>,
    Json(frm): Json<CreateForm>,
) -> Result<Json<i32>> {
    let id = model::insert(
        &state.pool,
        &model::Category {
            name: frm.name,
            ..Default::default()
        },
    )
    .await?;
    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct EditForm {
    pub id: i32,
    pub name: String,
}

pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<EditForm>,
) -> Result<Json<u64>> {
    let aff = model::update(
        &state.pool,
        &model::Category {
            id: frm.id,
            name: frm.name,
        },
    )
    .await?;
    Ok(Json(aff))
}

pub async fn delete(State(state): State<ArcAppState>, Path(id): Path<i32>) -> Result<Json<u64>> {
    let aff = model::delete(&state.pool, id).await?;
    Ok(Json(aff))
}

pub async fn find(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<model::Category>>> {
    let m = model::find(&state.pool, id).await?;
    Ok(Json(m))
}

pub async fn list(State(state): State<ArcAppState>) -> Result<Json<Vec<model::Category>>> {
    let ls = model::list(&state.pool).await.unwrap();
    Ok(Json(ls))
}
