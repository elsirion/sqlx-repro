use sqlx::{Any, AnyPool, query_as};
use sqlx::any::install_default_drivers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    install_default_drivers();
    let db = AnyPool::connect("sqlite::memory:").await?;
    let mut connection = db.acquire().await?;

    // 4294967297 = 2^32
    let number = query_as::<_, (i64,)>("SELECT 4294967296")
        .fetch_one(connection.as_mut())
        .await?
        .0;

    assert_eq!(number, 4294967296);

    Ok(())
}
