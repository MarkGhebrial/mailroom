//! Represents the database that stores user information.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use email_address::EmailAddress;
use log::{info, trace};
use sea_orm::{ActiveValue, Database, DatabaseConnection, DbErr, EntityTrait};

//use super::err::DbError;
use super::*;
use crate::config_helpers::get_all_addresses;
use crate::CONFIG;

/// Start up the database, modifying it if the configuration has changed and
/// creating it if it doesn't yet exist.
pub async fn initialize_db() -> Result<DatabaseConnection, DbErr> {
    // Initialize connection with sqlite
    let db = Database::connect(&CONFIG.database.url).await?;

    // No extra initialization is needed for sqlite
    // TODO: support other databases
    // https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html

    for user in get_all_addresses() {
        let user_entry = User::find_by_id(user.to_string()).one(&db).await?;

        match user_entry {
            Some(_) => trace!("Found user {}", user),
            None => {
                // The user is not yet in the database, so add the user.

                // Hash the default password of "password"
                let salt = SaltString::generate(&mut OsRng);
                let password_hash = Argon2::default()
                    .hash_password("password".as_bytes(), &salt)
                    .expect("Could not hash password")
                    .to_string();

                // Insert into the User table
                let new_user = user::ActiveModel {
                    email_address: ActiveValue::Set(user.to_string()),
                    password: ActiveValue::Set(password_hash),
                };
                User::insert(new_user).exec(&db).await?;

                info!("Created user with address {}", user);
            }
        }
    }

    Ok(db)
}

pub async fn get_user(address: EmailAddress) -> Option<User> {
    None
}
/*
pub fn authenticate_user(address: EmailAddress, password: String) -> Result<User, DbError> {
    // Load hashed password from DB
        // If it doesn't exist, hash the password and save that

    // Verify that the password matches
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

}*/
