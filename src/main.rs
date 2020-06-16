use dotenv;
use kv_log_macro as log;

use sqlx::Pool;
use sqlx::PgPool;
use sqlx::query;

#[async_std::main]
async fn main() -> Result<(), Error>{    
    dotenv::dotenv().ok();
    femme::with_level(femme::LevelFilter::Trace);
    log::info!("Listening on port 8080");
    
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool: PgPool  = Pool::new(&db_url).await?;
    let rows = query!("select 1 as one").fetch(&db_pool).await?;
    dbg!(rows);

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello world!") });
    app.listen("127.0.0.1:8000").await?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}