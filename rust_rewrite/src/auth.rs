use password_worker::*;
use crate::{Result, Error};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use jsonwebtoken::{encode, Header, EncodingKey};
/// i think this works, but i'm not sure

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
  let cost = 12;
  let max_threads = 4;
  let password_worker = PasswordWorker::new_bcrypt(max_threads)?;
  let is_valid = password_worker.verify(pwd, dbpwd).await?;

  Ok(is_valid)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // subject
    pub exp: usize, // expiration time
}

/// create_token takes a username string and returns a token string
/// the token is generated using the username and a secret key
pub fn create_token(username: &str) -> Result<String> {
  // sets claims for token
  // sub is the subject, which is the username
  // exp is expiry time, 4000 secs = 1 hour approx.
  let token_claims = Claims {
    sub: username.to_owned(),
    exp: 4000,
  };

  // encodes the token using the claims and a secret key
  let token = encode(&Header::default(), &token_claims, &EncodingKey::from_secret("secret".as_ref()))?;
  // returns a Result with the token
  Ok(token)
}

#[derive(Debug, Serialize, Deserialize)]
