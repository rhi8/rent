use std::fmt;
use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)] // Added Clone here
pub enum Platform {
    Playstation3,
    Playstation4,
    Playstation5,
    Xbox,
    InvalidPlatform
}

impl Platform {
   pub fn str_to_enum(value: &str) -> Platform {
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