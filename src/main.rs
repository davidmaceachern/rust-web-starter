use dotenv;
use kv_log_macro as log;

use tide::Server;
use tide::Request;
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

    let mut app: Server<State> = Server::with_state(State  { db_pool });

    app.at("/").get(|req: Request<State>| async move { 
        let db_pool:&PgPool = &req.state().db_pool;

        let rows = query!("select 1 as one").fetch_one(db_pool).await?;
        dbg!(rows);

        Ok("Hello world!") 
    });

    // app.at("/status").get(async move { Ok(rows) });
    
    app.listen("127.0.0.1:8000").await?;

    Ok(())
}
#[derive(Debug)]
struct State {
    db_pool: PgPool,
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