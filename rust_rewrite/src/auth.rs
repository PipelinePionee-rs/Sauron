use password_worker::*;
use crate::{Result, Error};
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

