use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx_pg::{category, post as post_mod, session, topic, user, AppState, ArcAppState};
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

    let topic_router = Router::new()
        .route("/", get(topic::handler::list).post(topic::handler::create))
        .route("/{id}/meta", get(topic::handler::meta))
        .route("/{id}/author", get(topic::handler::author))
        .route("/{id}/views", get(topic::handler::views))
        .route(
            "/{id}/increment-views",
            get(topic::handler::increment_views),
        );

    let session_router = Router::new()
        .route("/", post(session::handler::create))
        .route("/{token}", get(session::handler::find_by_token))
        .route("/email/{email}", get(session::handler::find_by_email));

    Router::new()
        .nest("/category", category_router)
        .nest("/user", user_router)
        .nest("/post", post_router)
        .nest("/topic", topic_router)
        .nest("/session", session_router)
        .with_state(state)
}
