use jsonwebtoken::Header;
//use log::Level::Error;
use rocket::http::Cookie;
use rocket::http::hyper::body::HttpBody;
use serde::{Serialize, Deserialize};
use sqlx::{Postgres, Transaction,Error};
use crate::enums::subscription_enum::SubscriptionType;
use crate::utils::id_generator::generate_uuid;
use crate ::enums::roles_enum::RolesEnum;
use crate::persistence::postgres_db::PostgresDbPool;
use crate::utils::password_helper::hash_password;
use crate::auth::authentication::generate_jwt;


#[derive(Serialize, Deserialize)]
pub struct RegisterCustomer {
    pub reference: Option<String>,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub subscription_type: SubscriptionType,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerCredentials {
    pub email: String,
    pub password: String,
    pub role: RolesEnum,
    pub reference: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerProfile {
    pub reference: String,
    pub email: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub subscription_type: SubscriptionType,
}

#[derive(Serialize, Deserialize)]
struct  Login {
    email: String,
    password: String,
}



static REF_STR: &str = "customer_";

impl RegisterCustomer {
    pub  async fn post_user(&self) -> Result<String, Error> {
        let reference = generate_uuid(REF_STR);
        let pool = &PostgresDbPool::global().pg_pool;
        let mut tx: Transaction<Postgres> = pool.begin().await?;

        let mut password_tobe_hashed  = &self.password;
        let hashed_password = hash_password(password_tobe_hashed);
        //validation

        let customer_registration_query = r#"
        INSERT INTO customer_profile (reference, name, address, phone, email, subscription_type)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        let customer_registration_query_result= sqlx::query(customer_registration_query)
            .bind(&reference)
            .bind(&self.name)
            .bind(&self.address)
            .bind(&self.phone)
            .bind(&self.email)
            .bind(&self.subscription_type.to_string())
            .execute(pool)
            .await?;

        let customer_credentials_query = r#"
        INSERT INTO customer_credentials (reference, email, password, role)
        VALUES ($1, $2, $3, $4)
        "#;

        let customer_credentials_query_result = sqlx::query(customer_credentials_query)
            .bind(&reference)
            .bind(&self.email)
            .bind(hashed_password)
            .bind(RolesEnum::Customer.to_string())
            .execute(pool)
            .await.unwrap();

        tx.commit().await?;

        let customer_registration_response = customer_registration_query_result.rows_affected();
        let customer_credentials_query_response = customer_credentials_query_result.rows_affected();

        if ( customer_credentials_query_response > 0 && customer_registration_response > 0) {
            println!("Query was successful for customer registration: {}", customer_registration_response);
            println!("Query was successful for customer credentials: {}", customer_credentials_query_response);
        }else {
            println!("Query was unsuccessful for customer credentials: {}", customer_credentials_query_response);
        }

        let jwt = generate_jwt(reference.clone(), self.subscription_type.clone(),RolesEnum::Customer ).unwrap();

        // let cookie = Cookie::build(("auth_token", jwt))
        //     .http_only(true) // Prevents JavaScript access
        //     .secure(true)    // Use only over HTTPS
        //     .path("/")       // Accessible at the root
        //     .max_age(time::Duration::hours(4))
        //     .build();

        Ok(jwt)



    }


}






