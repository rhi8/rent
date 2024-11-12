use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::serde::{Deserialize, Serialize};
use crate::enums::roles_enum::RolesEnum;
use crate::enums::subscription_enum::SubscriptionType;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub customer_ref: String,
    pub subscription_type: SubscriptionType,
    pub role: RolesEnum,
    pub exp: i64,
}

const SECRET_KEY: &str = "YOUR JWT SECRET KEY";

pub struct AuthenticateUser {
    pub token: String,
}

impl AuthenticateUser {
    pub fn generate_jwt(customer_ref: String, subscription_type: SubscriptionType, role: RolesEnum, ) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(4))
            .expect("Timestamp invalid")
            .timestamp();

        let claims = Claims {
            customer_ref,
            subscription_type,
            role,
            exp: expiration
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref()))
    }

    pub fn decode_token(&self) -> Result<TokenData<Claims> , jsonwebtoken::errors::Error> {
        decode::<Claims>(
            &self.token,
            &DecodingKey::from_secret(SECRET_KEY.as_ref()),
            &Validation::default(),
        )
    }

    pub fn is_valid (claims: &Claims) -> bool {
        claims.exp > Utc::now().timestamp()
    }
}







