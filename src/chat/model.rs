use futures::StreamExt;
use sqlx::PgExecutor;

const CHANNEL_NAME: &str = "CHAT_ROOM";
pub async fn notify(e: impl PgExecutor<'_>, msg: &str) -> sqlx::Result<()> {
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(CHANNEL_NAME)
        .bind(msg)
        .execute(e)
        .await?;
    Ok(())
}

pub async fn notify_process(dsn: String) -> sqlx::Result<()> {
    let mut listener = sqlx::postgres::PgListener::connect(&dsn).await?;
    listener.listen(CHANNEL_NAME).await?;

    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            println!("got message: {:?}", msg);
        }
    });
    Ok(())
}
