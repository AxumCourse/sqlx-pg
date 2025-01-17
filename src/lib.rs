pub mod category;
mod err;
pub mod user;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub type ArcAppState = std::sync::Arc<AppState>;
