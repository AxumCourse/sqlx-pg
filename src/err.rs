use axum::response::IntoResponse;

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl Error {
    pub fn new(msg: &str) -> Self {
        Self(anyhow::anyhow!("{}", msg))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.0.to_string().into_response()
    }
}
