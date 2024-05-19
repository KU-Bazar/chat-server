use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn db_init() -> Result<PgPool, Box<dyn std::error::Error>> {
    let url = "postgres://admin:password@localhost:5432/postgres";
    let connection_pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
    Ok(connection_pool)
}

pub async fn db_migration(conn: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::migrate!("./migrations").run(conn).await?;
    Ok(())
}
