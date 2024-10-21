use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};
use crate::persistence::postgres_db::PostgresDbPool;
use crate::utils::id_generator::generate_uuid;
use sqlx::{Transaction, query, Postgres, Error, Row, query_as, FromRow, PgPool};
use sqlx::Executor; // Importing Executor
use sqlx::postgres::{PgQueryResult, PgRow};
use std::fmt;
use std::str::FromStr;
use rocket::futures::future::err;
use rocket::futures::TryFutureExt;
use rocket::http::ext::IntoCollection;
use rocket::http::Status;
use crate::models;
use serde_json::Value; // For JSON handling
use crate::enums::subcription_enum::SubscriptionType;



//platform methods
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

#[derive(Debug, Serialize, Deserialize, Clone,FromRow)] // Added Clone here
pub struct Game {
    pub reference: Option<String>, // primary key
    pub name: String, // god of war
    pub description: String, // this is a game that ...
    pub game_category: Vec<String>, // action adventure
    pub subscription_type: Vec<SubscriptionType>, // basic standard
    pub inventory: Option<Vec<GameItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone,FromRow)] // Added Clone here
pub struct GameItem {
    pub reference: Option<String>,
    pub barcode: String,
    pub platform: Platform, // e.g., "PlayStation 4" or "Xbox"
    pub is_available: bool, // availability status of this specific copy
}

static REF_STR: &str = "game_";

impl Game {
    pub async fn edit_game(&self) ->  Result<Option<Game>, Error> {
        let pool = &PostgresDbPool::global().pg_pool; // Assuming you have a global pool
        let mut tx: Transaction<Postgres> = pool.begin().await?;

        let global_reference  = self.reference.as_ref().expect("Game must have a reference");

        // Update the game in the 'games' table, excluding reference
        let game_result: PgRow = query(
            r#"
            UPDATE games
            SET
                name = $1,
                description = $2,
                game_category = $3,
                subscription_type = $4
            WHERE reference = $5
            RETURNING *
            "#
        )
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.game_category)
            .bind(&self.subscription_type.iter().map(|st| st.to_string()).collect::<Vec<_>>())
            .bind(self.reference.as_ref().expect("Game must have a reference")) // Reference cannot be changed
            .fetch_one(tx.as_mut())
            .await?;

        let mut game_item_results:Vec<GameItem> = Vec::new();

        // Update inventory items associated with this game
        if let Some(inventory) = &self.inventory {
            for game_item in inventory {
                let updated_item: PgRow = query(
                    r#"
                INSERT INTO games_item_table (barcode, reference, platform, is_available)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (barcode) DO UPDATE
                SET platform = EXCLUDED.platform,
                is_available = EXCLUDED.is_available
                RETURNING *
                "#).bind(&game_item.barcode) // Barcode to identify the item
                    .bind(&global_reference) // Foreign key reference
                    .bind(&game_item.platform.to_string()) // Ensure platform is a string
                    .bind(game_item.is_available)
                    .fetch_one(tx.as_mut())
                    .await?;

                // Collect the updated game items
                game_item_results.push(GameItem {
                    reference: Some(updated_item.get("reference")), // Adjust as needed
                    barcode: updated_item.get("barcode"),
                    platform: Platform::str_to_enum(updated_item.get("platform")),
                    is_available: updated_item.get("is_available"),
                });
            }
        }

        // Commit the transaction
        tx.commit().await?;

        // Convert subscription types back to enum
        let convert_sub_type: Vec<SubscriptionType> = game_result.get::<Vec<String>, _>("subscription_type")
            .iter()
            .map(|st| SubscriptionType::str_to_enum(st))
            .collect();

        // Return the updated Game object
        Ok(Some(Game {
            reference: game_result.get("reference"),
            name: game_result.get("name"),
            description: game_result.get("description"),
            game_category: game_result.get("game_category"),
            subscription_type: convert_sub_type,
            inventory: Some(game_item_results), // Return the updated inventory
        }))
    }


        // Commit the transaction


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
            .bind(&self.subscription_type.iter().map(|st| st.to_string()).collect::<Vec<_>>()) // Convert SubscriptionType to strings // Convert SubscriptionType to strings
            .fetch_one(tx.as_mut()) // Correctly using `tx` here
            .await?;

        let mut game_item_results:Vec<GameItem> = Vec::new();

        // Check if the game has an inventory (Vec<GameItem>)
        if let Some(inventory) = &self.inventory {
            for game_item in inventory {

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

        tx.commit().await?;

        let convert_sub_type: Vec<SubscriptionType> = game_result.get::<Vec<String>, _>("subscription_type")
            .iter()
            .map(|st| SubscriptionType::str_to_enum(st))
            .collect();

        Ok(Some(Game {
            reference: Some(game_result.get("reference")),
            name: game_result.get("name"),
            description: game_result.get("description"),
            game_category: game_result.get("game_category"),
            subscription_type: convert_sub_type,
            inventory: Some(game_item_results),
        }))
    }
}

impl Game {
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

        let rows = sqlx::query_as::<_, (Option<String>, String, String, Vec<String>, Vec<String>, Option<serde_json::Value>)>(query) // Change `String` to `Vec<String>`
            .fetch_all(pool)
            .await?;

        let games = rows.into_iter().map(|(reference, name, description, game_category, subscription_type, inventory)| {

            let convert_sub_type = subscription_type
                .iter()
                .map(|st| SubscriptionType::str_to_enum(st))
                .collect::<Vec<_>>(); // Collecting into a Vec<SubscriptionType>

            let inventory_items = inventory.map(|inv| {
                serde_json::from_value::<Vec<GameItem>>(inv).unwrap_or_default() // Deserialize into Vec<GameItem>
            });

            Game {
                reference,
                name,
                description,
                game_category,
                subscription_type: convert_sub_type, // Correctly assign the converted subscription types
                inventory: inventory_items, // Safely parse inventory
            }
        }).collect();

        Ok(games)
    }
}


impl Game {
    pub async fn get_game_by_reference(reference: String) -> Result<Option<Game>, Error> {
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
    WHERE
        g.reference = $1
    GROUP BY
        g.reference, g.name, g.description, g.game_category, g.subscription_type
    "#;

        // Fetch the game details based on the reference
        let game_details: Option<PgRow> = sqlx::query(query)
            .bind(&reference)
            .fetch_optional(pool)
            .await?;

        match game_details {
            Some(row) => {
                // Extract the fields from the row
                let inventory_json: Option<Value> = row.get("inventory");

                let inventory = inventory_json
                    .and_then(|inv| serde_json::from_value::<Vec<GameItem>>(inv).ok()); // Deserialize into Vec<GameItem>

                // Convert subscription type from Vec<String> to Vec<SubscriptionType>
                let subscription_type_json: Vec<String> = row.get("subscription_type");
                let subscription_type = subscription_type_json
                    .iter()
                    .map(|st| SubscriptionType::str_to_enum(st))
                    .collect::<Vec<_>>();

                let game = Game {
                    reference: row.get("reference"),
                    name: row.get("name"),
                    description: row.get("description"),
                    game_category: row.get("game_category"),
                    subscription_type, // Assign the converted subscription types
                    inventory,
                };

                Ok(Some(game)) // Return the constructed Game object
            }
            None => {
                Err(Error::RowNotFound) // Change this according to your error handling strategy
            }
        }
    }
}
