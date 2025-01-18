use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{ArcAppState, Result};

use super::model;

#[derive(Deserialize)]
pub struct LoginForm {
    pub nickname: String,
}
pub async fn login(
    State(state): State<ArcAppState>,
    Json(frm): Json<LoginForm>,
) -> Result<Json<String>> {
    let msg = format!("欢迎 {} 进入聊天室", frm.nickname);
    model::notify(&state.pool, &msg).await?;

    Ok(Json(format!("你好，{}", frm.nickname)))
}
