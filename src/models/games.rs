use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};
use crate::persistence::postgres_db::PostgresDbPool;
use crate::utils::id_generator::generate_uuid;
use sqlx::{Transaction, query, Postgres, Error, Row, query_as};
use sqlx::Executor; // Importing Executor
use sqlx::postgres::{PgQueryResult, PgRow};
use std::fmt;
use std::str::FromStr;
use rocket::futures::future::err;
use crate::models;
use serde_json::Value; // For JSON handling


#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone here
enum SubscriptionType {
    Basic,
    Standard,
    Premium,
    InvalidSubscription
}
impl SubscriptionType {
    fn str_to_enum(value: &str) -> SubscriptionType {
        match value {
            "Basic" => SubscriptionType::Basic,
            "Standard" => SubscriptionType::Standard,
            "Premium" => SubscriptionType::Premium,
            _ => SubscriptionType::InvalidSubscription,
        }
    }
}


#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)] // Added Clone here
enum Platform {
    Playstation3,
    Playstation4,
    Playstation5,
    Xbox,
    InvalidPlatform
}

impl Platform {
    fn str_to_enum(value: &str) -> Platform {
        match value {
            "Playstation3" => Platform::Playstation3,
            "Playstation4" => Platform::Playstation4,
            "Playstation5" => Platform::Playstation5,
            "Xbox" => Platform::Xbox,
            _ => Platform::InvalidPlatform
        }
    }
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
    pub platform: Platform, // e.g., "PlayStation 4" or "Xbox"
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
            SubscriptionType::InvalidSubscription => "InvalidSubscription",
        };
        write!(f, "{}", subscription_str)
    }
}

// Implement the Display trait for Platform
impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let platform_str = match self {
            Platform::Playstation3 => "Playstation3",
            Platform::Playstation4 => "Playstation4",
            Platform::Playstation5 => "Playstation5",
            Platform::Xbox => "Xbox",
            Platform::InvalidPlatform => "InvalidPlatform"
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
        let game_result: PgRow = query(
            r#"
            INSERT INTO games (reference, name, description, game_category, subscription_type)
            VALUES ($1, $2, $3, $4, $5)
             RETURNING *
            "#
        )
            .bind(&reference)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.game_category)
            .bind(self.subscription_type.to_string()) // Assuming `to_string` is implemented for `SubscriptionType`
            .fetch_one(tx.as_mut()) // Correctly using `tx` here
            .await?;


        let mut game_item_results:Vec<GameItem> = Vec::new();

        // Check if the game has an inventory (Vec<GameItem>)
        if let Some(inventory) = &self.inventory {
            for game_item in inventory {
               // let item_reference = game_item.reference.clone().unwrap_or_else(|| generate_uuid(REF_STR));

                // Insert each game item into the 'games_item_table'
                let game_item: PgRow = query(
                    r#"
                    INSERT INTO games_item_table (barcode, reference, platform, is_available)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                    "#
                )
                    .bind(&game_item.barcode)                     // Barcode of the game item
                    .bind(&reference)                             // Reference to the 'games' table
                    .bind(&game_item.platform.to_string()) // Platform (as a TEXT array)
                    .bind(game_item.is_available)                // Availability status
                    .fetch_one(tx.as_mut()) // Execute the query
                    .await?;

                game_item_results.push(GameItem{
                    reference: game_item.get("reference"),
                    barcode: game_item.get("barcode"),
                    platform: Platform::str_to_enum( game_item.get("platform")),
                    is_available: game_item.get("is_available")
                });

            }

        }
        // Commit the transaction after all inserts
        tx.commit().await?;

        let convert_sub_type = game_result.get("subscription_type");

        println!("Game and game items inserted successfully with reference: {}", reference);
        Ok(Some(Game {
            reference: Some(game_result.get("reference")),
            name: game_result.get("name"),
            description: game_result.get("description"),
            game_category: game_result.get("game_category"),
            subscription_type: SubscriptionType::str_to_enum(convert_sub_type),
            inventory: Some(game_item_results),
        }))
    }

    pub async fn get_all_games() -> Result<Vec<Game>, sqlx::Error> {
        let pool = &PostgresDbPool::global().pg_pool;

        let query = r#"
    SELECT
        g.reference,
        g.name,
        g.description,
        g.game_category,  -- This is stored as an array in the DB
        g.subscription_type,
        json_agg(
            json_build_object(
                'reference', gi.reference,
                'barcode', gi.barcode,
                'platform', gi.platform,
                'is_available', gi.is_available
            )
        ) AS inventory
    FROM
        games g
    LEFT JOIN
        games_item_table gi ON g.reference = gi.reference
    GROUP BY
        g.reference, g.name, g.description, g.game_category, g.subscription_type
    "#;

        let rows = sqlx::query_as::<_, (Option<String>, String, String, Vec<String>, String, Option<serde_json::Value>)>(query)
            .fetch_all(pool)
            .await?;

        let games = rows.into_iter().map(|(reference, name, description, game_category, subscription_type, inventory)| {
            Game {
                reference,
                name,
                description,
                game_category,  // Now correctly deserialized as Vec<String>
                subscription_type: SubscriptionType::str_to_enum(&subscription_type),
                inventory: inventory.map(|inv| serde_json::from_value(inv).unwrap_or_default()),  // Safely parse inventory
            }
        }).collect();

        Ok(games)
    }

}
