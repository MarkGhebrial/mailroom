//! Represents the database that stores user information.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use email_address::EmailAddress;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr};
use tokio_postgres::{Client, NoTls};

use super::err::DbError;
use super::user::User;
use crate::CONFIG;

/// Start up the database, modifying it if the configuration has changed and
/// creating it if it doesn't yet exist.
pub async fn initialize_db() -> Result<DatabaseConnection, DbErr> {
    // Initialize connection with sqlite
    let db = Database::connect(&CONFIG.database.url).await?;

    // No extra initialization is needed for sqlite
    // TODO: support other databases
    // https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html

    Ok(db)
}

pub async fn get_user(address: EmailAddress) -> Result<User, DbError> {
    Ok(User)
}
/*
pub fn authenticate_user(address: EmailAddress, password: String) -> Result<User, DbError> {
    // Load hashed password from DB
        // If it doesn't exist, hash the password and save that

    // Verify that the password matches
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

}*/
