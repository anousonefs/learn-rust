#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticated(creds: Credentials) {
    if let Status::Connected = database::connect_db() {
        auth_utils::login(creds)
    }
}
