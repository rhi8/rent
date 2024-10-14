mod models;
mod persistence;
use rocket::launch;
use crate::persistence::postgres_db::PostgresDbPool;
use crate::persistence::postgres_db;
use crate::persistence::init_database::create_initial_table;

#[launch]
async fn rocket() -> _ {
    let db_pool: PostgresDbPool = PostgresDbPool::get_db_pool().await;
    postgres_db::INSTANCE.set(db_pool);

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

}
