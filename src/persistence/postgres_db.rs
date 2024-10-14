use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sqlx::{Error, Pool, Postgres};

#[derive(Debug)]
pub struct PostgresDbPool {
    pub pg_pool: Pool<Postgres>,
}


pub static INSTANCE: OnceCell<PostgresDbPool> = OnceCell::new();

impl PostgresDbPool {
    pub fn global() -> &'static PostgresDbPool {
        INSTANCE.get().expect("Database pool is not initialized")
    }

    pub async fn init_postgres_db_pool() -> Result<Pool<Postgres>, Error> {
        dotenv().ok();

        let database_url = std::env::var("DB_URL").expect("a database name is required");

        print!(" print!(database_url database_url);");
        print!("database_url: {}",database_url);

        let pool = sqlx::postgres::PgPool::connect(&database_url)
            .await?;

        Ok(pool)
    }

    pub async fn get_db_pool() -> PostgresDbPool {
        let pool = Self::init_postgres_db_pool().await.expect("Db pool is expected");

        let db_pool = PostgresDbPool { pg_pool: pool };

        return db_pool;
    }
}
