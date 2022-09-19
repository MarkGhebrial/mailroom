use std::str::FromStr;

use crate::CONFIG;
use email_address::{EmailAddress};

/// Get a list of all the email addresses in the configuration file. Panics
/// if any of them are invalid.
pub fn get_all_addresses() -> Vec<EmailAddress> {
    let mut out = vec![];
    for d in &CONFIG.domains {
        for u in &d.users {
            out.push(
                EmailAddress::from_str(
                    &format!("{}@{}", u, d.name)
                ).expect(&format!("Address \'{}@{}\' is invalid", u, d.name))
            );
        }
    }
    out
}