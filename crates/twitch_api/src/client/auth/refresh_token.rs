use chrono::{Duration, NaiveDateTime, Utc};
use openidconnect::reqwest::async_http_client;
use openidconnect::{OAuth2TokenResponse, RefreshToken, Scope};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::client::TwitchApiClient;
use crate::error::{OidcError, TwitchApiClientError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefreshTokenResponse {
    pub client_id: String,
    pub access_token: String,
    pub access_token_expiration: NaiveDateTime,
    pub refresh_token: String,
    pub scopes: HashSet<String>,
}

impl TwitchApiClient {
    pub async fn refresh_token(
        &self,
        refresh_token: String,
        scopes: HashSet<String>,
    ) -> Result<RefreshTokenResponse, TwitchApiClientError> {
        let refresh_token = RefreshToken::new(refresh_token);

        let mut refresh_request = self.client.exchange_refresh_token(&refresh_token);
        for scope in scopes.clone() {
            refresh_request = refresh_request.add_scope(Scope::new(scope));
        }

        let refresh_response = refresh_request
            .request_async(async_http_client)
            .await
            .map_err(OidcError::from)?;

        Ok(RefreshTokenResponse {
            client_id: self.client_id.clone(),
            access_token: refresh_response.access_token().secret().to_string(),
            access_token_expiration: Utc::now().naive_utc()
                + Duration::from_std(refresh_response.expires_in().unwrap()).unwrap(),
            refresh_token: refresh_response
                .refresh_token()
                .unwrap()
                .secret()
                .to_string(),
            scopes: scopes.clone(),
        })
    }
}
