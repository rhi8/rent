use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};
use crate::persistence::postgres_db::PostgresDbPool;
use crate::utils::id_generator::generate_uuid;
use sqlx::postgres::PgQueryResult;
use std::fmt;
use sqlx::{Executor, PgPool, Postgres, Transaction};


#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub reference: String,//primary key
    pub name: String,//god of war
    pub description: String,// this is a game that ...
    pub game_category: Vec<String>,//action adventure
    pub subscription_type: SubscriptionType,//basic standard
    pub inventory :Option<Vec<GameItem>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameItem {
    pub reference: Option<String>,
    pub barcode: String,
    pub platform: Vec<Platform>,   // e.g., "PlayStation 4" or "Xbox
    pub is_available: bool,  // availability status of this specific copy
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


impl Game {


    pub async fn post_game (&self) -> Result<Option<Game>, sqlx::Error>{

        let reference = generate_uuid(REF_STR);

        let pool = &PostgresDbPool::global().pg_pool;
        let mut tx = pool.begin().await?;

        // Insert the new game into the 'games' table
        let game_result: PgQueryResult = sqlx::query(
            r#"
        INSERT INTO games (reference, name, description, game_category, subscription_type)
        VALUES ($1, $2, $3, $4, $5)
        "#
        )
            .bind(&reference)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.game_category)
            .bind(&self.subscription_type.to_string())
            .execute(&mut tx)
            .await?;

        if let Some(inventory) = &self.inventory {
            for game_item in inventory {
                let item_reference = game_item.reference.clone().unwrap_or_else(|| generate_uuid(REF_STR));

                // Insert each game item into the 'games_item_table'
                let game_item_result: PgQueryResult = sqlx::query(
                    r#"
                    INSERT INTO games_item_table (barcode, reference, platform, is_available)
                    VALUES ($1, $2, $3, $4)
                    "#
                )
                    .bind(&game_item.barcode)                // Barcode of the game item
                    .bind(&reference)                        // Reference to the 'games' table
                    .bind(&game_item.platform.iter().map(|p| p.to_string()).collect::<Vec<_>>())  // Platform (as a TEXT array)
                    .bind(&game_item.is_available)           // Availability status
                    .execute(&mut tx)
                    .await?;
            }


            tx.commit().await?;

            println!("Game and game items inserted successfully with reference: {}", reference);
            Ok(Some(Game {
                reference,
                name: self.name.clone(),
                description: self.description.clone(),
                game_category: self.game_category.clone(),
                subscription_type: self.subscription_type.clone(),
                inventory: self.inventory.clone(),
            }))
        }








    }


}



