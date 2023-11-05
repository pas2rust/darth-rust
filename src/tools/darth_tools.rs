use crate::tools::random_bytes::random_bytes;
use chrono::{DateTime, Duration, Local, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::new_smtp_lettre_send_email::new_smtp_lettre_send_email;
pub struct DarthTools;

pub trait DarthToolsTrait {
    fn new_lettre_send_email(
        from_name: &str,
        from_email: &str,
        to_name: &str,
        to_email: &str,
        user_smtp: &str,
        password_smtp: &str,
        provider: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), lettre::error::Error>;
    fn new_jsonwebtoken_hs256_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs384_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs512_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs256_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs384_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs512_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn new_bcrypt_hash(value: String, cost: u32) -> Result<String, bcrypt::BcryptError>;
    fn new_bcrypt_verify(value: String, hash: &String) -> Result<bool, bcrypt::BcryptError>;
    fn new_date_utc_add_time_by_weeks(weeks: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_days(days: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_hours(hours: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_minutes(minutes: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_seconds(seconds: i64) -> DateTime<Utc>;
    fn new_date_local_add_time_by_weeks(weeks: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_days(days: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_hours(hours: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_minutes(minutes: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_seconds(seconds: i64) -> DateTime<Local>;
    fn new_uuid() -> String;
    fn new_date_utc_now() -> DateTime<Utc>;
    fn new_date_local_now() -> DateTime<Local>;
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub items: Value,
    pub expiration: usize,
}

impl DarthToolsTrait for DarthTools {
    fn new_lettre_send_email(
        from_name: &str,
        from_email: &str,
        to_name: &str,
        to_email: &str,
        user_smtp: &str,
        password_smtp: &str,
        provider: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), lettre::error::Error> {
        new_smtp_lettre_send_email(
            from_name,
            from_email,
            to_name,
            to_email,
            user_smtp,
            password_smtp,
            provider,
            subject,
            body,
        )
    }
    fn new_jsonwebtoken_hs256_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs384_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS384),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs512_decode(
        token: String,
        secret: String,
    ) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs256_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let my_claims = Token { items, expiration };
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }
    fn new_jsonwebtoken_hs384_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let my_claims = Token { items, expiration };
        let header = Header::new(jsonwebtoken::Algorithm::HS384);
        encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }
    fn new_jsonwebtoken_hs512_encode(
        items: Value,
        expiration: usize,
        secret: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let my_claims = Token { items, expiration };
        let header = Header::new(jsonwebtoken::Algorithm::HS512);
        encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }
    fn new_bcrypt_hash(value: String, cost: u32) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(value, cost)
    }
    fn new_bcrypt_verify(value: String, hash: &String) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(value, hash)
    }
    fn new_date_utc_add_time_by_hours(hours: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::hours(hours);
        now + duration
    }
    fn new_date_utc_add_time_by_minutes(minutes: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::minutes(minutes);
        now + duration
    }
    fn new_date_utc_add_time_by_seconds(seconds: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::seconds(seconds);
        now + duration
    }
    fn new_date_utc_add_time_by_weeks(weeks: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::weeks(weeks);
        now + duration
    }
    fn new_date_utc_add_time_by_days(days: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::days(days);
        now + duration
    }
    fn new_date_local_add_time_by_minutes(minutes: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::minutes(minutes);
        now + duration
    }
    fn new_date_local_add_time_by_weeks(weeks: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::weeks(weeks);
        now + duration
    }
    fn new_date_local_add_time_by_seconds(seconds: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::seconds(seconds);
        now + duration
    }
    fn new_date_local_add_time_by_hours(hours: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::hours(hours);
        now + duration
    }
    fn new_date_local_add_time_by_days(days: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::days(days);
        now + duration
    }
    fn new_date_local_now() -> DateTime<Local> {
        Local::now()
    }
    fn new_date_utc_now() -> DateTime<Utc> {
        Utc::now()
    }
    fn new_uuid() -> String {
        Uuid::new_v4().to_string()
    }
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String {
        random_bytes(
            gen_uppercase,
            gen_lowercase,
            gen_number,
            gen_special_characters,
            gen_emoji,
        )
    }
}
