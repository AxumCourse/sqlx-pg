use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{ArcAppState, Error, Result};

use super::model;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
}
pub async fn register(
    State(state): State<ArcAppState>,
    Json(frm): Json<RegisterForm>,
) -> Result<Json<i32>> {
    let mut tx = state.pool.begin().await?;
    let id = match model::register(
        &mut tx,
        &model::User {
            username: frm.username,
            ..Default::default()
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e);
        }
    };
    tx.commit().await?;
    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct EditForm {
    pub id: i32,
    pub username: String,
}
pub async fn edit(
    State(state): State<ArcAppState>,
    Json(frm): Json<EditForm>,
) -> Result<Json<u64>> {
    let exists = model::exists(&state.pool, &frm.username, Some(frm.id)).await?;
    if exists {
        return Err(Error::new("用户已存在"));
    }

    let aff = model::update(
        &state.pool,
        &model::User {
            id: frm.id,
            username: frm.username,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(aff))
}

pub async fn edit_by_tx(
    State(state): State<ArcAppState>,
    Json(frm): Json<EditForm>,
) -> Result<Json<u64>> {
    let mut tx = state.pool.begin().await?;
    let exists = match model::exists(&mut *tx, &frm.username, Some(frm.id)).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    if exists {
        return Err(Error::new("用户已存在"));
    }

    let aff = match model::update(
        &mut *tx,
        &model::User {
            id: frm.id,
            username: frm.username,
            ..Default::default()
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    tx.commit().await?;

    Ok(Json(aff))
}
