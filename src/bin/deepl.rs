use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
#[derive(Serialize, Deserialize, Debug)]
struct Translations {
    translations: Vec<Translation>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Translation {
    detected_source_language: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut params = HashMap::new();
    let auth_key = env::var("AUTH_KEY").unwrap();
    params.insert("auth_key", auth_key);
    params.insert("text", String::from("こんにちは"));
    params.insert("target_lang", String::from("EN"));
    let response = reqwest::Client::new()
        .post("https://api-free.deepl.com/v2/translate")
        .form(&params)
        .send()
        .await?
        .json::<Translations>()
        .await?;
    println!("{:?}", response);

    Ok(())
}
