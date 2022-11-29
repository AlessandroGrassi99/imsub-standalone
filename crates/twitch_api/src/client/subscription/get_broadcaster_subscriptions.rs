use serde::{Deserialize, Serialize};

use crate::client::{HasPagination, Pagination, TwitchApiClient};
use crate::error::TwitchApiClientError;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BroadcasterSubscription {
    pub broadcaster_id: String,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub is_gift: bool,
    pub gifter_login: Option<String>,
    pub gifter_name: Option<String>,
    pub tier: String,
    pub gifter_id: Option<String>,
    pub plan_name: String,
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBroadcasterSubscriptions {
    pub data: Vec<BroadcasterSubscription>,
    pub pagination: Pagination,
    pub total: u64,
    pub points: u64,
}

impl HasPagination for GetBroadcasterSubscriptions {
    type D = BroadcasterSubscription;

    fn pagination(&self) -> Pagination {
        self.pagination.clone()
    }

    fn data(&mut self) -> &mut Vec<Self::D> {
        &mut self.data
    }
}

impl TwitchApiClient {
    pub async fn get_broadcaster_subscriptions(
        &self,
        broadcaster_id: String,
        access_token: String,
    ) -> Result<Vec<BroadcasterSubscription>, TwitchApiClientError> {
        let params = vec![
            ("broadcaster_id".to_string(), broadcaster_id.to_string()),
            ("first".to_string(), "100".to_string()),
        ];

        let subs = self
            .get_elems::<GetBroadcasterSubscriptions, BroadcasterSubscription>(
                "https://api.twitch.tv/helix/subscriptions",
                self.build_headers(access_token),
                params,
            )
            .await?;

        Ok(subs)
    }
}
