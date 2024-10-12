use dotenv::dotenv;
use jsonwebtoken::{encode, get_current_timestamp, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // username
    iat: u64,    // Issued at
    exp: u64,    // Expiration
}

pub fn create_jwt(username: String) -> Result<String, jsonwebtoken::errors::Error> {
    // Load secret key from .env file
    dotenv().ok();
    let secret: &str = &std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    let claims = Claims {
        sub: username,                                  // username
        iat: get_current_timestamp(),                   // Current timestamp
        exp: get_current_timestamp() + (3600 * 24 * 3), // Expire in 3 days
    };

    let header = Header::default();
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

// Returns the username if the token is valid
pub fn validate_jwt(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Load secret key from .env file
    dotenv().ok();
    let secret: &str = &std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims.sub)
}
