#![allow(dead_code, unused_variables)]


mod database {
    pub enum Status {
        Connected,
        Interrupted
    }
    pub fn connect_to_db() -> Status{
        return Status::Connected
    }
    
    pub fn get_user() {
        // get user
    }
}

mod auth_utils {

    mod models {
        pub struct Credentials {
            username: String,
            password: String
        }
        
    }

    pub fn login(creds: models::Credentials) {
        // auth
        crate::database::get_user();
    }
    
    pub fn logout() {
        // logout
    }

}

// use declaration
use auth_utils::models::Credentials;
use database::Status;

// pub fn authenticate(creds: auth_utils::models::Credentials) {
pub fn authenticate(creds: Credentials) {
    // if let database::Status::Connected = database::connect_to_db() {
    if let Status::Connected = database::connect_to_db() {
        auth_utils::login(creds);
    }
}