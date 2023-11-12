use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::darth_tools::DarthTools;
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub items: Value,
    pub expiration: usize,
}

pub trait JsonwebtokenTrait {
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
}

impl JsonwebtokenTrait for DarthTools {
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
}
