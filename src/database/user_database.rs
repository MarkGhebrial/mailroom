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

pub async fn db_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect(&CONFIG.database.url).await
}

/// Start up the database, modifying it if the configuration has changed and
/// creating it if it doesn't yet exist.
pub async fn initialize_db() -> Result<DatabaseConnection, DbErr> {
    // Initialize connection with sqlite
    let db = db_connection().await?;

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

/// Look up a user in the database. If the user is not found, return
/// `None`
pub async fn get_user(address: &EmailAddress) -> Result<Option<user::Model>, DbErr> {
    let db = db_connection().await?;

    let user = User::find_by_id(address.to_string()).one(&db).await?;
    Ok(user)
}

/// Look up user up in the database and check that that `password` is
/// correct. If the user does not exist or the password is wrong, `None`
/// is returned
pub async fn authenticate_user(
    address: &EmailAddress,
    password: &str,
) -> Result<Option<user::Model>, DbErr> {
    // Look up the user
    let user = get_user(address).await?;
    let hashed_password = match &user {
        Some(model) => &model.password,
        None => return Ok(None),
    };

    let parsed_hash = PasswordHash::new(&hashed_password)
        .expect(&format!("Invalid password hash: {}", hashed_password));

    // Verify that the password matches
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(Some(user.unwrap())) // Unrapping is OK here because we checked earlier that user is `Some`
    } else {
        Ok(None)
    }
}
