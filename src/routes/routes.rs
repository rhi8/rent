//use rocket::serde::json::Json;
use rocket::{get, http::Status, State};
use rocket::{post, serde::json::Json};
use sqlx::PgPool;
use crate::models::games::Game;


#[post("/postgames", data = "<game>", format = "application/json")]
pub async fn post_games(game: Json<Game>) -> Result<Option<Json<Game>>, Status> {
    match game.post_game().await {
        Ok(Some(saved_game)) => {
            println!("{:?}", saved_game);
            Ok(Some(Json(saved_game))) // Return the saved game wrapped in Json
        },
        Ok(None) => {
            // If the game was not saved for some reason
            Err(Status::InternalServerError)
        },
        Err(e) => {
            // Handle the error and return a Status
            eprintln!("Error posting game: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/getgames", format = "application/json")]
pub async fn get_all_games_route() -> Result<Json<Vec<Game>>, Status> {
    match Game::get_all_games().await {
        Ok(games) => {
            // Return the games wrapped in a Json type
            Ok(Json(games))
        },
        Err(e) => {
            // Log the error for debugging
            eprintln!("Error fetching games: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}





