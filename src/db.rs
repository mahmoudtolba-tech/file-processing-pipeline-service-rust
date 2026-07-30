use sqlx::{Pool, Postgres, migrate::Migrator};
use std::path::Path;

static MIGRATOR: Migrator = Migrator::new(std::path::Path::new("./migrations"));

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await?;
    Ok(())
}