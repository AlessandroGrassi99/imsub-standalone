use serde::{Deserialize, Serialize};

use crate::prelude::{HasPagination, Pagination, TwitchApiClient, TwitchApiClientError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Moderator {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetModerators {
    pub data: Vec<Moderator>,
    pub pagination: Pagination,
}

impl HasPagination for GetModerators {
    type D = Moderator;

    fn pagination(&self) -> Pagination {
        self.pagination.clone()
    }

    fn data(&mut self) -> &mut Vec<Self::D> {
        &mut self.data
    }
}

impl TwitchApiClient {
    pub async fn get_moderators(
        &self,
        broadcaster_id: String,
        access_token: String,
    ) -> Result<Vec<Moderator>, TwitchApiClientError> {
        let params = vec![
            ("broadcaster_id".to_string(), broadcaster_id.to_string()),
            ("first".to_string(), "100".to_string()),
        ];

        let mods = self
            .get_elems::<GetModerators, Moderator>(
                "https://api.twitch.tv/helix/moderation/moderators",
                self.build_headers(access_token),
                params,
            )
            .await?;

        Ok(mods)
    }
}
