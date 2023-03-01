use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::{TwitchApiClient, TwitchApiClientError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Subscription {
    pub broadcaster_id: String,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub is_gift: bool,
    pub gifter_login: Option<String>,
    pub gifter_name: Option<String>,
    pub tier: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CheckUserSubscription {
    pub data: Vec<Subscription>,
}

impl TwitchApiClient {
    pub async fn check_user_subscription(
        &self,
        user_id: String,
        access_token: String,
    ) -> Result<Subscription, TwitchApiClientError> {
        let url = Url::parse_with_params(
            "https://api.twitch.tv/helix/subscriptions/user",
            &[("broadcaster_id", self.broadcast_id.as_str()), ("user_id", user_id.as_str())],
        )
        .map_err(TwitchApiClientError::from)?;

        let headers = self.build_headers(access_token);

        let res = self.build_get_request(url, headers).await?;
        if res.status() != StatusCode::OK {
            use TwitchApiClientError::*;

            let err = match res.status() {
                StatusCode::BAD_REQUEST => TwitchApiInvalidRequest,
                StatusCode::NOT_FOUND => TwitchApiUserNotSubscribed,
                StatusCode::UNAUTHORIZED => TwitchApiAuthorizationFailed,
                _ => TwitchApiOther(res.status().to_string()),
            };
            return Err(err);
        }

        let sub: CheckUserSubscription = res
            .json()
            .await
            .map_err(|err| TwitchApiClientError::ResponseError(err.to_string()))?;
        let sub = sub
            .data
            .get(0)
            .cloned()
            .ok_or(TwitchApiClientError::TwitchApiUserNotSubscribed)?;
        Ok(sub)
    }
}
