//! Represents the database that stores user information.

use email_address::EmailAddress;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use tokio_postgres::{NoTls, Client};

use super::err::DbError;
use super::user::User;

/// Start up the database, modifying it if the configuration has changed and
/// creating it if it doesn't yet exist.
pub async fn initialize_db() -> Result<Client, tokio_postgres::Error> {
    
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=mail password=Login123", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    //client.query(statement, params)

    Ok(client)
}

pub fn get_user(address: EmailAddress) -> Result<User, DbError> {
    Ok(User)
}
/*
pub fn authenticate_user(address: EmailAddress, password: String) -> Result<User, DbError> {
    // Load hashed password from DB
        // If it doesn't exist, hash the password and save that
    
    // Verify that the password matches
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

}*/