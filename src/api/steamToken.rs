use serde::Deserialize;
use std::fs;
use reqwest::Client;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    pub domain: String,
    pub api_key: String,
}

pub async fn read_config(config_path: String) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

pub async fn steam_get_owned_games(api_key: &str, steamid: &str) -> Result<Value, reqwest::Error> {
    let url = format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json&include_appinfo=1&include_played_free_games=1", api_key, steamid);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let json = response.json().await?;
    Ok(json)
}

pub async fn steam_get_recently_played_games(api_key: &str, steamid: &str) -> Result<Value, reqwest::Error> {
    let url = format!("http://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v0001/?key={}&steamid={}&format=json", api_key, steamid);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let json = response.json().await?;
    Ok(json)
}