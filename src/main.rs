use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx_pg::{category, post as post_mod, user, AppState, ArcAppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dsn = std::env::var("DATABASE_URL").unwrap_or("".into());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await?;
    let state = Arc::new(AppState { pool });

    let app = router_init(state);

    let listener = TcpListener::bind("0.0.0.0:9527").await?;

    axum::serve(listener, app).await?;
    Ok(())
}

fn router_init(state: ArcAppState) -> Router {
    let category_router = Router::new()
        .route(
            "/",
            get(category::handler::list)
                .post(category::handler::create)
                .put(category::handler::edit),
        )
        .route(
            "/{id}",
            get(category::handler::find).delete(category::handler::delete),
        );

    let user_router = Router::new().route(
        "/",
        post(user::handler::register)
            .put(user::handler::edit)
            .patch(user::handler::edit_by_tx),
    );

    let post_router = Router::new()
        .route("/", post(post_mod::handler::create))
        .route("/{id}", get(post_mod::handler::find));

    Router::new()
        .nest("/category", category_router)
        .nest("/user", user_router)
        .nest("/post", post_router)
        .with_state(state)
}
