pub mod auth;
pub mod moderation;
pub mod request;
pub mod subscription;
pub mod user;

pub mod prelude {
    pub use super::Pagination;

    pub use super::HasPagination;

    pub use super::auth::prelude::*;
    pub use super::moderation::prelude::*;
    pub use super::subscription::prelude::*;
    pub use super::user::prelude::*;
    pub use super::request::prelude::*;
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pagination {
    pub cursor: Option<String>,
}

pub trait HasPagination {
    type D;

    fn pagination(&self) -> Pagination;

    fn data(&mut self) -> &mut Vec<Self::D>;
}
