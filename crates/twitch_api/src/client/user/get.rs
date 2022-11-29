use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::TwitchApiClient;
use crate::error::TwitchApiClientError;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub broadcaster_type: String,
    pub description: String,
    pub display_name: String,
    pub id: String,
    pub login: String,
    pub offline_image_url: String,
    pub profile_image_url: String,
    pub r#type: String,
    pub view_count: i64,
    pub email: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetUsers {
    pub data: Vec<User>,
}

impl TwitchApiClient {
    pub async fn get_user(
        &self,
        user_id: String,
        access_token: Option<String>,
    ) -> Result<Option<User>, TwitchApiClientError> {
        let url = Url::parse_with_params("https://api.twitch.tv/helix/users", &[("id", user_id)])
            .map_err(TwitchApiClientError::from)?;

        let headers = match access_token {
            Some(access_token) => self.build_headers(access_token),
            None => HeaderMap::new(),
        };

        let res = self.build_get_request(url, headers).await?;

        let users: GetUsers = res
            .json()
            .await
            .map_err(|err| TwitchApiClientError::ResponseError(err.to_string()))?;
        let user = users.data.get(0).cloned();

        Ok(user)
    }
}
