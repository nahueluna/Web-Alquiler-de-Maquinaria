use chrono::{Datelike, NaiveDate, Utc, Duration};
use hex;
use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use std::env;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool, Manager};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData, encode, Header, EncodingKey};
use crate::custom_types::{structs::Claims, enums::RunningEnv};
use crate::constants::*;

pub fn generate_random_string(lenght: usize) -> String {
    let random_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect();

    random_string
}

pub fn create_2fa_code() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(10000..1000000)
}

pub async fn create_pool(running_env: RunningEnv) -> Pool {

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

pub fn encode_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", salt, password).as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn send_mail(address: &str, subject: &str, body: &str) -> Result<(), String> {
    let email_address = env::var("EMAIL").map_err(|e| e.to_string())?;
    let app_password = env::var("APP_PASSWORD").map_err(|e| e.to_string())?;

    let email = Message::builder()
        .from(Mailbox::new(
            Some("SAGA Soporte".into()),
            email_address.parse().map_err(|_| "Email parsing failed")?,
        ))
        .reply_to(email_address.parse().map_err(|_| "Email parsing failed")?)
        .to(address.parse().map_err(|_| "Email parsing failed")?)
        .subject(subject)
        .singlepart(SinglePart::plain(body.to_string()))
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(email_address.clone(), app_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn validate_jwt(jwt: &str) -> Option<TokenData<Claims>> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JTW_SECRET_KEY must be set in the .env file");

    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    ).ok()
}

pub fn generate_jwt(user_id: i32, role: i16, is_refresh: bool) -> Result<String, ()> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JTW_SECRET_KEY must be set in the .env file");

    let exp_option = if is_refresh {
        Utc::now().checked_add_signed(Duration::days(REFRESH_EXPIRATION_DAYS))
    } else {
        Utc::now().checked_add_signed(Duration::minutes(ACCESS_EXPIRATION_MINUTES))
    };

    let exp = match exp_option {
        Some(e) => e.timestamp() as usize,
        None => return Err(()),
    };

    let claims = Claims {
        user_id,
        exp,
        role,
        is_refresh,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).map_err(|_| ())
}
