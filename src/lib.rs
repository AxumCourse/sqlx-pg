pub mod category;
pub mod chat;
mod err;
pub mod post;
pub mod session;
pub mod topic;
pub mod user;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub type ArcAppState = std::sync::Arc<AppState>;
