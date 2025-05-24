use chrono::{Datelike, Duration, NaiveDate, Utc};
use dotenvy::dotenv;
use hex;
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use std::env;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool, Manager};

use crate::custom_types::{structs::*, enums::RunningEnv};

pub fn generate_random_string(lenght: usize) -> String {
    let random_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect();

    random_string
}

pub async fn create_pool(running_env: RunningEnv) -> Pool {
    dotenv().ok();

    let database_url = match running_env {
        RunningEnv::Production => env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file"),
        RunningEnv::Testing => env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set in .env file"),
    };

    let pg_config: tokio_postgres::Config = database_url.parse().expect("Invalid database URL");
    let mgr = Manager::new(pg_config, NoTls);

    Pool::builder(mgr)
        .max_size(10)
        .build()
        .expect("Failed to create pool")

}

pub fn is_adult(birth_date: NaiveDate) -> bool {
    let today = Utc::now().naive_utc().date();

    let mut age = today.year() - birth_date.year();

    if today.ordinal() < birth_date.ordinal() {
        age -= 1;
    }

    age >= 18
}

pub fn encode_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn generate_token(
    user_id: &str,
    role: &str,
    token_type: &str,
    expires_in_minutes: usize,
    secret_key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in_minutes as i64))
        .expect("date out of range")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        role: role.to_string(),
        token_type: token_type.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
}

// Checks if a password with salt is correct
pub fn check_password(password_from_input: &str, stored_password_hash: &str) -> bool {
    return encode_password(password_from_input) == stored_password_hash;
}

// Gets an user's password hash from the database
pub async fn get_password_hash_from_db(email: &str) -> String {
    let hardcoed_password_hash = "$argon2i$v=19$m=65536,t=4,p=1$randomsalt$randomhash";

    hardcoed_password_hash.to_string()
}

pub fn send_mail(addressee: &str, subject: &str, body: &str) {
    // Load variables from .env
    dotenv().ok();

    let email_address = env::var("EMAIL").unwrap();
    let app_password = env::var("APP_PASSWORD").unwrap();

    let email = Message::builder()
        .from(Mailbox::new(
            Some("Rust Bot".into()),
            email_address.parse().unwrap(),
        ))
        .reply_to(email_address.parse().unwrap())
        .to(addressee.parse().unwrap())
        .subject(subject)
        .singlepart(SinglePart::plain(body.to_string()))
        .unwrap();

    let creds = Credentials::new(email_address.clone(), app_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {e}"),
    }
}
