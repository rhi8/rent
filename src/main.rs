mod models;
mod persistence;
mod utils;
mod routes;
mod enums;

use rocket::{launch, routes};
use crate::persistence::postgres_db::PostgresDbPool;
use crate::persistence::postgres_db;
use crate::persistence::init_database::create_initial_table;
use crate::routes::routes::{post_games, get_all_games_route, get_single_games_route, edit_games_route};

#[launch]
async fn rocket() -> _ {
    let db_pool: PostgresDbPool = PostgresDbPool::get_db_pool().await;
    let _ = postgres_db::INSTANCE.set(db_pool);

    //initiate table
    match create_initial_table().await {
        Ok(_) => {
            println!("Database initialized successfully!");
        }
        Err(e) => {
            println!("Failed to initialize the database: {}", e);
        }
    }




    rocket::build()
        .mount("/", routes![post_games,get_all_games_route,get_single_games_route,edit_games_route])

}
