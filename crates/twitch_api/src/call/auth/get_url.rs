use cached::Cached;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::prelude::{TwitchApiClient, CachedAuthRequest};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthUrl {
    pub request_id: String,
    pub url: String,
    pub scopes: HashSet<String>,
    pub state: String,
    pub nonce: String,
}

impl TwitchApiClient {
    pub fn get_auth_url(
        &self,
        request_id: String,
        state: Option<String>,
        mut scopes: HashSet<String>,
    ) -> AuthUrl {
        let csrf_token = move || match state.clone() {
            Some(value) => CsrfToken::new(value),
            None => CsrfToken::new_random_len(96),
        };

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256_len(96);

        let mut auth_request = self.client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            csrf_token,
            Nonce::new_random,
        );

        scopes.insert("openid".to_string());
        for scope in scopes.clone() {
            auth_request = auth_request.add_scope(Scope::new(scope));
        }
        auth_request = auth_request
            .add_extra_param("claims".to_string(), r#"{"id_token":{"email":null,"email_verified":null},"userinfo":{"preferred_username":null, "picture":null}}"#.to_string())
            .set_pkce_challenge(pkce_challenge);

        let (auth_url, csrf_token, nonce) = auth_request.url();

        {
            let cached_request = CachedAuthRequest::new(
                request_id.clone(),
                csrf_token.clone(),
                nonce.clone(),
                pkce_verifier.secret().to_string(),
                scopes.clone(),
            );
            let cache_arc = self.cache_requests.clone();
            let mut cache_lock = cache_arc.write().unwrap();
            cache_lock.cache_set(request_id.clone(), cached_request);
        }

        AuthUrl {
            request_id,
            url: auth_url.to_string(),
            scopes: scopes.clone(),
            state: csrf_token.secret().to_string(),
            nonce: nonce.secret().to_string(),
        }
    }
}
