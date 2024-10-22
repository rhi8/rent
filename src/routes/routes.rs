//use rocket::serde::json::Json;
use rocket::{get, http::Status, put, State};
use rocket::{post, serde::json::Json};
use sqlx::{Error, PgPool};
use crate::models::games::Game;
use crate::models::customer::{RegisterCustomer};



#[post("/registeruser", data = "<customer_details>", format = "application/json")]
pub async fn register_customer_route(customer_details: Json<RegisterCustomer>) -> Result<Json<String>, Status> {
    match customer_details.post_user().await {
        Ok(jwt) => {
            // Return the JWT token as a JSON response on success
            Ok(Json(jwt))
        }
        Err(e) => {
            // Log the error and return an appropriate HTTP status code
            eprintln!("Error registering user: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}




#[put("/editgames", data = "<game>", format = "application/json")]
pub async fn edit_games_route(game: Json<Game>) -> Result<Option<Json<Game>>, Status> {
    match game.edit_game().await {
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

#[get("/getgames/<reference>", format = "application/json")]
pub async fn get_single_games_route(reference: String) -> Result<Json<Game>, Status> {
    match Game::get_game_by_reference(reference).await {
        Ok(Some(game)) => {
            // Return the game wrapped in a Json type
            Ok(Json(game))
        },
        Ok(None) => {
            // If no game was found, return a 404 Not Found status
            Err(Status::NotFound)
        },
        Err(e) => {
            // Log the error for debugging (consider using a logging framework)
            eprintln!("Error fetching game: {:?}", e);
            // Return a 500 Internal Server Error status for any unexpected errors
            Err(Status::InternalServerError)
        }
    }
}






