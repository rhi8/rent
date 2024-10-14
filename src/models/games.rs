use std::collections::HashSet;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;



#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
enum SubscriptionType{
    Basic,
    Standard,
    Premium
}
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
enum Platform{
    Playstation3,
    Playstation4,
    Playstation5,
    Xbox
}
#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[derive(Hash)]
struct GameItem {
    pub platform: Platform,
    pub available_copy: u8,
    pub barcode_id: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub  name: String,
    pub id: Option<i32>,
    pub description: String,
    pub available_to: Vec<SubscriptionType>,
    pub game_availability: HashSet<GameItem>,
}



