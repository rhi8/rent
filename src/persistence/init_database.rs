use sqlx::Error;
use crate::persistence::postgres_db::PostgresDbPool;

pub async fn create_initial_table() -> Result<(), Error> {
    let pool = &PostgresDbPool::global().pg_pool;



    let customer_credentials = r#"
    CREATE TABLE IF NOT EXISTS customer_credentials (
        reference VARCHAR PRIMARY KEY,
        email VARCHAR UNIQUE NOT NULL,
        password VARCHAR NOT NULL,
        role VARCHAR NOT NULL
    )
    "#;

    sqlx::query(customer_credentials)
        .execute(pool)
        .await?;

     let customer_profile = r#"
 CREATE TABLE IF NOT EXISTS customer_profile (
     reference VARCHAR PRIMARY KEY,
     name VARCHAR NOT NULL,
     address VARCHAR NOT NULL,
     phone VARCHAR NOT NULL,
     email VARCHAR UNIQUE NOT NULL,
     subscription_type VARCHAR NOT NULL,
     FOREIGN KEY (reference) REFERENCES customer_credentials (reference) ON DELETE CASCADE
 )
 "#;

 sqlx::query(customer_profile)
     .execute(pool)
     .await?;


    let games_table = r#"
    CREATE TABLE IF NOT EXISTS games (
        reference VARCHAR PRIMARY KEY,
        name VARCHAR NOT NULL,
        description TEXT NOT NULL,
        game_category TEXT[],  -- Array of strings
        subscription_type TEXT[] NOT NULL
    )
    "#;

    sqlx::query(games_table)
        .execute(pool)
        .await?;



    let games_item_table = r#"
    CREATE TABLE IF NOT EXISTS games_item_table (
        barcode VARCHAR PRIMARY KEY,
        reference VARCHAR REFERENCES games(reference) ON DELETE CASCADE,
        platform TEXT,  -- Array of strings
        is_available BOOLEAN
    )
    "#;

    sqlx::query(games_item_table)
        .execute(pool)
        .await?;






    Ok(())


}