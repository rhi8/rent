use std::collections::HashSet;
use rocket::Error;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::persistence::postgres_db::PostgresDbPool;

#[derive(Serialize, Deserialize,, Debug)]
enum SubscriptionType{
    Basic,
    Standard,
    Premium
}
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
#[derive(Debug)]
enum Platform{
    Playstation3,
    Playstation4,
    Playstation5,
    Xbox
}

#[derive(Debug)]
pub struct Game {
    pub id: Option<i32>,//on adding to database primary key
    pub name: String,//god of war
    pub description: String,// this is a game that ...
    pub game_category: Vec<String>,//action adventure
    pub subscription_type: SubscriptionType,//basic standard
    pub inventory :Vec<GameItem>



}

#[derive(Debug)]
pub struct GameItem {
    pub platform: String,   // e.g., "PlayStation 4" or "Xbox"
    pub barcode: String,     // unique identifier for that specific copy
    pub is_available: bool,  // availability status of this specific copy
}







/*

impl Game<GameItem> {
    pub async fn new(id: Option<i32>, name: String,  description: String, available_to: Vec<SubscriptionType>) -> Game {
        Game {
            name,
            id,
            description,
            available_to,
            game_availability: Some(HashSet::new()), // Start with an empty set of game_availability
        }
    }

    pub async fn post_game (&self) -> Result<Option<Game>, sqlx::Error>{

        let pool = &PostgresDbPool::global().pg_pool;

        // Start a transaction
        let mut tx = pool.begin().await;

        // Insert the game
        let game_query = r#"
        INSERT INTO game (name, description, available_to)
        VALUES ($1, $2, $3)
        RETURNING id, name, description, available_to
    "#;

        let game = sqlx::query_as::<_, Game>(game_query)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.available_to)
            .fetch_one(&mut tx)
            .await?;

        // Insert availability records
        let availability_query = r#"
        INSERT INTO availability (game_id, game_item_id)
        VALUES ($1, $2)
    "#;

        for game_item in &self.game_availability {
            // Assuming GameItem has a method to get its ID (you may need to adjust this)
            let game_item_id = game_item.barcode_id; // Get the game_item_id from your GameItem struct

            sqlx::query(availability_query)
                .bind(game.id)
                .bind(game_item_id)
                .execute(&mut tx)
                .await?;
        }

        // Commit the transaction
        tx.commit().await?;

        Ok(Some(game))

    }


}

*/

