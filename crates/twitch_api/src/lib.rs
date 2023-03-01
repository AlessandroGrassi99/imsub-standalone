pub mod call;
pub mod error;
mod oidc_client;

pub mod prelude {
    pub use crate::call::prelude::*;
    pub use crate::TwitchApiClient;
    pub use crate::error::prelude::*;
    pub use crate::oidc_client::prelude::*;
}

use cached::TimedCache;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthType, ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use reqwest::{header::{HeaderMap, HeaderValue}, Response};
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};
use tracing::instrument;
use url::Url;

use prelude::*;

const ISSUER_URL: &str = "https://id.twitch.tv/oauth2";

#[derive(Clone, Debug)]
pub struct TwitchApiClient {
    pub client: CoreTwitchOidcClient,
    pub client_id: String,
    pub client_secret: String,
    pub cache_requests: Arc<RwLock<TimedCache<String, CachedAuthRequest>>>,
    pub http_client: reqwest::Client,
    pub broadcast_id: String,
}

impl TwitchApiClient {
    #[instrument(skip(client_secret), fields(fn_path = "TwitchApiClient::new"))]
    pub async fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        broadcast_id: String,
    ) -> Result<Self, TwitchApiClientError> {
        let issuer = IssuerUrl::new(ISSUER_URL.to_string()).map_err(TwitchApiClientError::from)?;

        let provider_metadata =
            CoreProviderMetadata::discover_async(issuer.clone(), async_http_client)
                .await
                .map_err(OidcError::from)?;

        let redirect_url = RedirectUrl::new(redirect_uri).map_err(TwitchApiClientError::from)?;
        let client = CoreTwitchOidcClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
        )
            .set_redirect_uri(redirect_url)
            .set_auth_type(AuthType::RequestBody);

        let cache_requests = Arc::new(RwLock::new(TimedCache::with_lifespan(360000)));

        Ok(Self {
            client,
            client_id,
            client_secret,
            cache_requests,
            http_client: reqwest::Client::new(),
            broadcast_id,
        })
    }

    async fn build_get_request(
        &self,
        url: Url,
        headers: HeaderMap,
    ) -> Result<Response, TwitchApiClientError> {
        let client_id = HeaderValue::from_str(self.client_id.clone().as_str()).unwrap();

        let req = self
            .http_client
            .get(url)
            .headers(headers)
            .header("Client-Id", client_id);
        let res = req
            .send()
            .await
            .map_err(|err| TwitchApiClientError::RequestError(err.to_string()))?;

        Ok(res)
    }

    async fn build_post_request(
        &self,
        url: Url,
        headers: HeaderMap,
    ) -> Result<Response, TwitchApiClientError> {
        let client_id = HeaderValue::from_str(self.client_id.clone().as_str()).unwrap();

        let req = self
            .http_client
            .post(url)
            .headers(headers)
            .header("Client-Id", client_id);
        let res = req
            .send()
            .await
            .map_err(|err| TwitchApiClientError::RequestError(err.to_string()))?;

        Ok(res)
    }

    fn build_headers(&self, access_token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {}", access_token);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(auth.as_str()).unwrap());
        headers
    }

    pub async fn get_elems<T: HasPagination<D = K> + DeserializeOwned, K>(
        &self,
        url: &str,
        headers: HeaderMap,
        params: Vec<(String, String)>,
    ) -> Result<Vec<K>, TwitchApiClientError> {
        let mut vec_res: Vec<K> = Vec::new();

        let mut tmp_params = params.clone();
        loop {
            let url =
                Url::parse_with_params(url, &tmp_params).map_err(TwitchApiClientError::from)?;

            let response = self.build_get_request(url.clone(), headers.clone()).await?;
            if response.status() != StatusCode::OK {
                use TwitchApiClientError::*;
                let err = match response.status() {
                    StatusCode::BAD_REQUEST => TwitchApiInvalidRequest,
                    StatusCode::UNAUTHORIZED => TwitchApiAuthorizationFailed,
                    _ => TwitchApiOther(response.status().to_string()),
                };
                return Err(err);
            }

            let mut res: T = response
                .json()
                .await
                .map_err(|err| TwitchApiClientError::ResponseError(err.to_string()))?;

            if res.data().is_empty() || res.pagination().cursor.is_none() {
                break;
            }

            tmp_params = params.clone();
            tmp_params.push((
                "after".to_string(),
                res.pagination().cursor.unwrap().clone(),
            ));

            vec_res.append(res.data());
        }

        Ok(vec_res)
    }
}
