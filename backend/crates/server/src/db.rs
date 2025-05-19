use sea_orm::{ConnectOptions, Database, DbConn, DbErr};
use std::{env, time::Duration};

pub(crate) async fn db_connect() -> Result<DbConn, DbErr> {
    let database_url =
        env::var("DATABASE_URL").map_err(|_| DbErr::Custom("DATABASE_URL is not set".into()))?;

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opt).await
}
