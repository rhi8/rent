use std::fmt;
use std::fmt::Display;
use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone here
pub enum SubscriptionType {
    Basic,
    Standard,
    Premium,
    InvalidSubscription
}

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

impl SubscriptionType {
    pub fn str_to_enum(value: &str) -> SubscriptionType {
        match value {
            "Basic" => SubscriptionType::Basic,
            "Standard" => SubscriptionType::Standard,
            "Premium" => SubscriptionType::Premium,
            _ => SubscriptionType::InvalidSubscription,
        }
    }
}