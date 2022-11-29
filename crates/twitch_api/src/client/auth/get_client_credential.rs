use chrono::NaiveDateTime;
use serde::Deserialize;
use url::Url;

use crate::client::TwitchApiClient;
use crate::TwitchApiClientError;

#[derive(Debug, Deserialize, Clone)]
pub struct GetClientCredential {
    pub access_token: String,
    #[serde(with = "from_u64_to_datetime")]
    pub expires_in: NaiveDateTime,
    pub token_type: String,
}

impl TwitchApiClient {
    pub async fn get_client_credential(&self) -> Result<GetClientCredential, TwitchApiClientError> {
        let url = Url::parse("https://id.twitch.tv/oauth2/token")?;
        let params = [
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", "client_credentials".to_string()),
        ];
        let req = self.http_client.post(url).form(&params);
        let res = req
            .send()
            .await
            .map_err(|err| TwitchApiClientError::RequestError(err.to_string()))?;

        let credential: GetClientCredential = res
            .json()
            .await
            .map_err(|err| TwitchApiClientError::ResponseError(err.to_string()))?;

        Ok(credential)
    }
}

mod from_u64_to_datetime {
    use chrono::{Duration, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(chrono::Utc::now().naive_utc() + Duration::seconds(i64::deserialize(deserializer)?))
    }
}
