#![allow(unused)]
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    // 1) Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:welcome@localhost/postgres")
        .await?;

     Ok(())

}
