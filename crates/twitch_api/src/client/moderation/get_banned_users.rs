use serde::{Deserialize, Serialize};

use crate::client::{HasPagination, Pagination, TwitchApiClient};
use crate::error::TwitchApiClientError;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BannedUser {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub expires_at: String,
    pub reason: String,
    pub moderator_id: String,
    pub moderator_login: String,
    pub moderator_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBannedUsers {
    pub data: Vec<BannedUser>,
    pub pagination: Pagination,
}

impl HasPagination for GetBannedUsers {
    type D = BannedUser;

    fn pagination(&self) -> Pagination {
        self.pagination.clone()
    }

    fn data(&mut self) -> &mut Vec<Self::D> {
        &mut self.data
    }
}

impl TwitchApiClient {
    pub async fn get_banned_users(
        &self,
        broadcaster_id: String,
        access_token: String,
    ) -> Result<Vec<BannedUser>, TwitchApiClientError> {
        let params = vec![
            ("broadcaster_id".to_string(), broadcaster_id.to_string()),
            ("first".to_string(), "100".to_string()),
        ];

        let bans = self
            .get_elems::<GetBannedUsers, BannedUser>(
                "https://api.twitch.tv/helix/moderation/banned",
                self.build_headers(access_token),
                params,
            )
            .await?;

        Ok(bans)
    }
}
