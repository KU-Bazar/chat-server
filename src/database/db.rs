use sqlx::PgPool;

pub async fn db_init(url: &str) -> Result<PgPool, Box<dyn std::error::Error>> {
    let connection_pool = sqlx::PgPool::connect(url).await?;
    Ok(connection_pool)
}

pub async fn db_migration(conn: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::migrate!("./migrations").run(conn).await?;
    Ok(())
}
