use sqlx::Error;
use sqlx::postgres::PgPoolOptions;
use crate::persistence::postgres_db::PostgresDbPool;

pub async fn create_initial_table() -> Result<(), Error> {
    let pool = &PostgresDbPool::global().pg_pool;

    let create_game = r#"
    CREATE TABLE IF NOT EXISTS game (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        available_to VARCHAR(20)[] NOT NULL
    )
    "#;

    sqlx::query(create_game)
        .execute(pool)
        .await?;




    let create_game_item_table = r#"
    CREATE TABLE IF NOT EXISTS game_item (
        id SERIAL PRIMARY KEY,
        platform VARCHAR(20) NOT NULL,
        available_copy SMALLINT NOT NULL
    )
    "#;

    sqlx::query(create_game_item_table)
        .execute(pool)
        .await?;



    let create_game_barcode_table = r#"
    CREATE TABLE IF NOT EXISTS barcode (
        id SERIAL PRIMARY KEY,
        barcode VARCHAR(255) NOT NULL,
        game_item_id INT REFERENCES game_item(id) ON DELETE CASCADE  -- Corrected reference
    )
    "#;

    sqlx::query(create_game_barcode_table)
        .execute(pool)
        .await?;



    let create_game_availability_table = r#"
    CREATE TABLE IF NOT EXISTS availability (
         game_id INT REFERENCES game(id) ON DELETE CASCADE,         -- Corrected reference
         game_item_id INT REFERENCES game_item(id) ON DELETE CASCADE, -- Corrected reference
         PRIMARY KEY (game_id, game_item_id)
    )
    "#;

    sqlx::query(create_game_availability_table)
        .execute(pool)
        .await?;

    Ok(())
}