pub mod client;
pub mod error;
mod oidc_client;

use std::collections::HashMap;

pub use client::TwitchApiClient;
pub use error::TwitchApiClientError;

pub mod prelude {
    pub use crate::client::{
        auth::prelude::*, moderation::prelude::*, subscription::prelude::*, user::prelude::*,
    };
}

#[derive(Debug)]
pub struct TwitchApiClientsManager {
    clients: HashMap<String, TwitchApiClient>,
}

impl TwitchApiClientsManager {
    pub async fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub async fn add(
        &mut self,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Result<(), TwitchApiClientError> {
        let twitch_client = TwitchApiClient::new(
            String::from("https://id.twitch.tv/oauth2"),
            client_id.clone(),
            client_secret,
            redirect_uri,
        )
        .await?;

        self.clients.insert(client_id, twitch_client);
        Ok(())
    }

    pub async fn get(&self, client_id: &str) -> Option<&TwitchApiClient> {
        Some(self.clients.get(client_id)?)
    }

    pub fn contains(&self, client_id: &str) -> bool {
        self.clients.contains_key(client_id)
    }
}
