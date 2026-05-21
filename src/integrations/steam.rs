use std::time::Duration;

use serde::Deserialize;

const CURRENT_PLAYERS_URL: &str = 
    "https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone)]
pub struct SteamClient {
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct CurrentPlayersResponse {
    response: CurrentPlayersPayload,
}

#[derive(Deserialize)]
struct CurrentPlayersPayload {
    player_count: u32,
}

impl SteamClient {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { http }
    }

    pub async fn current_players(&self, app_id: u32) -> u32 {
        let url = format!("{CURRENT_PLAYERS_URL}?appid={app_id}");

        let Ok(response) = self.http.get(url).send().await else {
            return 0;
        };

        let Ok(response) = response.error_for_status() else {
            return 0;
        };

        let Ok(payload) = response.json::<CurrentPlayersResponse>().await else {
            return 0;
        };

        payload.response.player_count
    }
}