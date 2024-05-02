extern crate chrono;

use validator::{Validate, ValidationError};
use chrono::{Local, DateTime};

use std::fs::File;
use std::io::Write;
use base64::{decode, encode};
use serde::{Deserialize, Serialize};

enum Roles{
    Subscriber,Admin,Editor,
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Login {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn new(user: User) -> User {
        User { username: user.username, email: user.email }
    }

    pub fn login(user: User) -> bool {
        let isLoginSuccessful = false;
        isLoginSuccessful
    }
    pub fn save_user(u: User) {
        let mut new_user = u;
        new_user.username = convertBase64ToFile(new_user.username);
        println!("{:?}", new_user);
    }
}

fn convertBase64ToFile(base64String: String) -> String {
    let now: DateTime<Local> = Local::now();
    let decoded_base64_data = decode(base64String).expect("Failed to decode Base64 string");
    let filename = format!("{}.png", now.timestamp());

    let mut file = File::create(&filename).expect("Failed to create file");
    file.write(&decoded_base64_data).expect("Failed to write to file");
    filename
}