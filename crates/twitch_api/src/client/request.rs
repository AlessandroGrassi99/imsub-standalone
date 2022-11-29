use openidconnect::{CsrfToken, Nonce};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CachedAuthRequest {
    pub request_id: String,
    pub state: CsrfToken,
    pub nonce: Nonce,
    pub pkce_verifier: String,
    pub scopes: HashSet<String>,
}

impl CachedAuthRequest {
    pub fn new(
        id: String,
        state: CsrfToken,
        nonce: Nonce,
        pkce_verifier: String,
        scopes: HashSet<String>,
    ) -> Self {
        Self {
            request_id: id,
            state,
            nonce,
            pkce_verifier,
            scopes,
        }
    }
}
