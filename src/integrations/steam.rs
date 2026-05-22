use serde::Deserialize;

use crate::integrations::http::HttpClient;

const CURRENT_PLAYERS_URL: &str =
    "https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/";

#[derive(Clone)]
pub struct SteamClient {
    http: HttpClient,
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
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn current_players(&self, app_id: u32) -> Result<u32, reqwest::Error> {
        let url = format!("{CURRENT_PLAYERS_URL}?appid={app_id}");

        let payload = self
            .http
            .raw()
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<CurrentPlayersResponse>()
            .await?;

        Ok(payload.response.player_count)
    }
}
