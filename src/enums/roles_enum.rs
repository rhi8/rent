use rocket::serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt;

#[derive(Serialize, Deserialize)] // Added Clone here
pub enum RolesEnum {
    Admin,
    Customer,
    InvalidRole
}

impl Display for RolesEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            RolesEnum::Admin => "Admin",
            RolesEnum::Customer => "Customer",
            RolesEnum::InvalidRole => "InvalidRole"
        };
        write!(f, "{}", role_str)
    }
}

impl RolesEnum {
    pub fn str_to_enum(value: &str) -> RolesEnum {
        match value {
            "Admin" => RolesEnum::Admin,
            "Customer" => RolesEnum::Customer,
            _ => RolesEnum::InvalidRole,
        }
    }
}