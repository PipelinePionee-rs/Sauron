use password_worker::*;
use crate::error::Result;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use jsonwebtoken::{encode, decode, DecodingKey, Validation, Header, EncodingKey};


// ---------------------------------------------------------------------------
// Password hashing / verification
// ---------------------------------------------------------------------------

// hash_password takes a password string and returns a hashed password string
pub async fn hash_password(pwd: &str) -> Result<String> {
  let cost = 12;
  let max_threads = 4;
  let password_worker = PasswordWorker::new_bcrypt(max_threads)?;
  let hashed_password = password_worker.hash(pwd, BcryptConfig { cost}).await?;
  Ok(hashed_password)
}

/// verify_password takes a password string and a hashed password string
/// and returns a boolean indicating whether the password is valid
/// pwd is the password from login attempt,
/// dbpwd is the hashed password from the database
pub async fn verify_password(pwd: &str, dbpwd: &str) -> Result<bool> {
  let max_threads = 4;
  let password_worker = PasswordWorker::new_bcrypt(max_threads)?;
  let is_valid = password_worker.verify(pwd, dbpwd).await?;

  Ok(is_valid)
}

// ---------------------------------------------------------------------------
// Token generation / verification
// ---------------------------------------------------------------------------

// Claims struct is used to encode and decode the token
// sub is the subject, which is the username
// exp is expiry time, 4000 secs = 1 hour approx.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // subject
    pub exp: usize, // expiration time
}

/// create_token takes a username string and returns a token string
/// the token is generated using the username and a secret key
pub fn create_token(username: &str) -> Result<String> {

  let token_claims = Claims {
    sub: username.to_owned(),
    exp: 4000,
  };

  // encodes the token using the claims and a secret key
  let token = encode(&Header::default(), &token_claims, &EncodingKey::from_secret("secret".as_ref()))?;
  // returns a Result with the token
  Ok(token)
}

// decode_token takes a token string and returns a Result with the Claims struct
// we will check the sub field of the Claims struct to verify the user
// if the sub(username) exists in the database, then the token is valid
pub fn decode_token(token: &str) -> Result<Claims> {
  let token = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
  // returns a Result with the Claims struct
  Ok(token.claims)
}