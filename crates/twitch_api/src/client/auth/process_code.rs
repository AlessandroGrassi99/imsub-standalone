use cached::Cached;
use chrono::NaiveDateTime;
use openidconnect::core::CoreUserInfoClaims;
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, OAuth2TokenResponse, PkceCodeVerifier, TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::client::TwitchApiClient;
use crate::error::{OidcError, TwitchApiClientError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProcessCodeRes {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub client_id: String,
    pub access_token: String,
    pub access_token_expiration: NaiveDateTime,
    pub refresh_token: String,
    pub token_scopes: HashSet<String>,
}

impl TwitchApiClient {
    pub async fn process_auth_code(
        &self,
        request_id: String,
        code: String,
        state: Option<String>,
        mut scopes: HashSet<String>,
    ) -> Result<ProcessCodeRes, TwitchApiClientError> {
        let cached_request = || {
            self.cache_requests
                .clone()
                .write()
                .unwrap()
                .cache_remove(&request_id)
                .ok_or(TwitchApiClientError::CachedRequestNotFound)
        };

        let cached_request = cached_request()?;

        if state.is_some() && *cached_request.state.secret() != state.clone().unwrap() {
            let msg = "State does not match".to_string();
            return Err(TwitchApiClientError::CachedRequestDoesNotMatch(msg));
        }
        scopes.insert("openid".to_string());
        if scopes != cached_request.scopes {
            let msg = "Scopes does not match".to_string();
            return Err(TwitchApiClientError::CachedRequestDoesNotMatch(msg));
        }

        let token_request = self
            .client
            .exchange_code(AuthorizationCode::new(code.clone()))
            .set_pkce_verifier(PkceCodeVerifier::new(cached_request.pkce_verifier.clone()));

        let token_response = token_request
            .request_async(async_http_client)
            .await
            .map_err(OidcError::from)?;

        let id_token = token_response
            .id_token()
            .ok_or(OidcError::IdTokenNotFound)?;
        let claims = id_token
            .claims(&self.client.id_token_verifier(), &cached_request.nonce)
            .map_err(OidcError::from)?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg().map_err(OidcError::from)?,
            )
            .map_err(OidcError::from)?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(TwitchApiClientError::from(OidcError::InvalidAccessToken));
            }
        }

        let user_info: CoreUserInfoClaims = self
            .client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(OidcError::from)?
            .request_async(async_http_client)
            .await
            .map_err(OidcError::from)?;

        let auth_res = ProcessCodeRes {
            id: user_info.subject().to_string(),
            username: user_info
                .preferred_username()
                .map(|username| username.to_string()),
            email: claims.email().map(|email| email.to_string()),
            client_id: self.client_id.clone(),
            access_token: token_response.access_token().secret().to_string(),
            access_token_expiration: claims.expiration().naive_utc(),
            refresh_token: token_response
                .refresh_token()
                .ok_or(TwitchApiClientError::ResponseError(
                    "refresh token not found".to_string(),
                ))?
                .secret()
                .to_string(),
            token_scopes: cached_request.scopes.clone(),
        };

        Ok(auth_res)
    }
}
