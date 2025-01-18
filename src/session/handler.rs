use std::collections::BTreeMap;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{ArcAppState, Result};

use super::model;

#[derive(Deserialize)]
pub struct CreateForm {
    pub data: BTreeMap<String, String>,
}
pub async fn create(
    State(state): State<ArcAppState>,
    Json(frm): Json<CreateForm>,
) -> Result<Json<String>> {
    let token = model::create(&state.pool, &model::Session::with_data(frm.data)).await?;
    Ok(Json(token))
}

pub async fn find_by_token(
    State(state): State<ArcAppState>,
    Path(token): Path<String>,
) -> Result<Json<Option<model::Session>>> {
    let session = model::find_by_token(&state.pool, &token).await?;
    Ok(Json(session))
}

pub async fn find_by_email(
    State(state): State<ArcAppState>,
    Path(email): Path<String>,
) -> Result<Json<Option<model::Session>>> {
    let session = model::find_by_email(&state.pool, &email).await?;
    Ok(Json(session))
}
