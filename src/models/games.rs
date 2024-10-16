use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};
use crate::persistence::postgres_db::PostgresDbPool;
use crate::utils::id_generator::generate_uuid;
use sqlx::{PgPool, Transaction, query, Postgres, Error};
use sqlx::Executor; // Importing Executor
use sqlx::postgres::PgQueryResult;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone here
enum SubscriptionType {
    Basic,
    Standard,
    Premium,
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)] // Added Clone here
enum Platform {
    Playstation3,
    Playstation4,
    Playstation5,
    Xbox,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone here
pub struct Game {
    pub reference: Option<String>, // primary key
    pub name: String, // god of war
    pub description: String, // this is a game that ...
    pub game_category: Vec<String>, // action adventure
    pub subscription_type: SubscriptionType, // basic standard
    pub inventory: Option<Vec<GameItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone here
pub struct GameItem {
    pub reference: Option<String>,
    pub barcode: String,
    pub platform: Vec<Platform>, // e.g., "PlayStation 4" or "Xbox"
    pub is_available: bool, // availability status of this specific copy
}

static REF_STR: &str = "game_";

// Implement the Display trait for SubscriptionType
impl Display for SubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let subscription_str = match self {
            SubscriptionType::Basic => "Basic",
            SubscriptionType::Standard => "Standard",
            SubscriptionType::Premium => "Premium",
        };
        write!(f, "{}", subscription_str)
    }
}

// Implement the Display trait for Platform
impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let platform_str = match self {
            Platform::Playstation3 => "Playstation 3",
            Platform::Playstation4 => "Playstation 4",
            Platform::Playstation5 => "Playstation 5",
            Platform::Xbox => "Xbox",
        };
        write!(f, "{}", platform_str)
    }
}

impl Game {
    pub async fn post_game(&self) -> Result<Option<Game>, sqlx::Error> {
        let reference = generate_uuid(REF_STR);
        let pool = &PostgresDbPool::global().pg_pool;
        let mut tx: Transaction<Postgres> = pool.begin().await?;

        // Insert the new game into the 'games' table
        let game_result: PgQueryResult = query(
            r#"
            INSERT INTO games (reference, name, description, game_category, subscription_type)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
            .bind(&reference)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.game_category)
            .bind(self.subscription_type.to_string()) // Assuming `to_string` is implemented for `SubscriptionType`
            .execute(tx.as_mut()) // Correctly using `tx` here
            .await?;

        // Check if the game has an inventory (Vec<GameItem>)
        if let Some(inventory) = &self.inventory {
            for game_item in inventory {
                let item_reference = game_item.reference.clone().unwrap();

                // Insert each game item into the 'games_item_table'
                let game_item_result: PgQueryResult = query(
                    r#"
                    INSERT INTO games_item_table (barcode, reference, platform, is_available)
                    VALUES ($1, $2, $3, $4)
                    "#
                )
                    .bind(&game_item.barcode)                     // Barcode of the game item
                    .bind(&reference)                             // Reference to the 'games' table
                    .bind(&game_item.platform.iter().map(|p| p.to_string()).collect::<Vec<_>>()) // Platform (as a TEXT array)
                    .bind(game_item.is_available)                // Availability status
                    .execute(tx.as_mut()) // Execute the query
                    .await?;
            }
        }

        // Commit the transaction after all inserts
        tx.commit().await?;

        println!("Game and game items inserted successfully with reference: {}", reference);
        Ok(Some(Game {
            reference: Some(reference),
            name: self.name.clone(),
            description: self.description.clone(),
            game_category: self.game_category.clone(),
            subscription_type: self.subscription_type.clone(),
            inventory: self.inventory.clone(),
        }))
    }
}
