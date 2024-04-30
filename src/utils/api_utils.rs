use std::collections::HashMap;
use serde_json::json;
use crate::models::user::{Login, User};

use tokio::time::{self, Duration};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;

use dotenv::dotenv;
use std::env;

pub async fn process_login(loginParams: Login) -> HashMap<i32,String> {
    let mut result: HashMap<i32,String> = HashMap::new();
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    dotenv().ok();
    let base_url = env::var("AD_URL").expect("AD environment variable");
    let payload = json!(loginParams).to_string();

    println!("base_url: {}",&base_url);

    let request = client
        .post(base_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload);
    let response = request.send().await;
    match response {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| String::new());

            println!("success: {}",format!("{}{}",status,&text));

            if status.is_success() {
                result.insert(200,text);
            } else {
                result.insert(500,text);
            }

        }
        Err(error) => {
            result.insert(100,error.to_string());
            println!("failure: {}",format!("{}",error.to_string()))
        }
    }
    result
}