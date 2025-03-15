mod api;

use clap::Parser;
use crate::api::steamToken::read_config;
use crate::api::steamToken::steam_get_owned_games;
use crate::api::steamToken::steam_get_recently_played_games;
use serde_json::Value;
use std::error::Error;
use chrono::NaiveDateTime;

#[derive(Parser)]
#[clap(name = "Steam Time", about = "A tool to get Steam game information")]
struct Args {
    #[clap(short = 'i', long="id", value_parser, help = "Steam ID")]
    steamid: String,

    #[clap(short = 'c', long="config", value_parser, default_value = "config.yaml", help = "Configuration file path")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = read_config(args.config).await?;
    println!("Domain: {}", config.domain);
    println!("API Key: {}", config.api_key);

    let steamid = args.steamid;

    let owned_games = steam_get_owned_games(&config.api_key, &steamid).await?;
    // println!("Owned Games: {:?}", owned_games);

    let recently_played_games = steam_get_recently_played_games(&config.api_key, &steamid).await?;
    // println!("Recently Played Games: {:?}", recently_played_games);

    save_to_csv("owned_games.csv", &owned_games)?;
    save_to_csv("recently_played_games.csv", &recently_played_games)?;

    Ok(())
}

fn save_to_csv(filename: &str, data: &Value) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;

    if let Some(mut games) = data["response"]["games"].as_array().cloned() {
        // Sort games by playtime_forever in descending order
        games.sort_by(|a, b| b["playtime_forever"].as_u64().unwrap_or(0).cmp(&a["playtime_forever"].as_u64().unwrap_or(0)));

        wtr.write_record(&["appid", "name", "playtime_forever", "last_played"])?;
        for game in games {
            let appid = game["appid"].as_u64().unwrap_or(0);
            let name = game["name"].as_str().unwrap_or("");
            let playtime_forever = game["playtime_forever"].as_u64().unwrap_or(0)/60;
            let rtime_last_played = game["rtime_last_played"].as_i64().unwrap_or(0);
            let last_played = NaiveDateTime::from_timestamp_opt(rtime_last_played, 0)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            wtr.write_record(&[
                appid.to_string(),
                name.to_string(),
                playtime_forever.to_string(),
                last_played,
            ])?;
        }
    }

    wtr.flush()?;
    Ok(())
}