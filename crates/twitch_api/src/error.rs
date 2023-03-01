pub mod prelude {
    pub use super::OidcError;
    pub use super::TwitchApiClientError;
}


use openidconnect::{
    ClaimsVerificationError, ConfigurationError, DiscoveryError, ErrorResponse, RequestTokenError,
    SigningError, UserInfoError,
};
use std::error::Error;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum OidcError {
    #[error("{0}")]
    StandardOidc(String),
    #[error("ID Token not found")]
    IdTokenNotFound,
    #[error("Invalid access token")]
    InvalidAccessToken,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TwitchApiClientError {
    #[error("Oidc error: {0}")]
    OidcError(String),
    #[error("Cached request not found")]
    CachedRequestNotFound,
    #[error("Cached request does not match: {0}")]
    CachedRequestDoesNotMatch(String),
    #[error("URL parse error: {0}")]
    UrlParseError(String),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Response error: {0}")]
    ResponseError(String),

    #[error("Twitch API - Invalid request")]
    TwitchApiInvalidRequest,
    #[error("Twitch API - Authorization failed")]
    TwitchApiAuthorizationFailed,
    #[error("Twitch API - Other error: {0}")]
    TwitchApiOther(String),
    // Get users
    #[error("Twitch API - User not found")]
    TwitchApiUserNotFound,
    // Subscription
    #[error("Twitch API - User not subscribed")]
    TwitchApiUserNotSubscribed,

    #[error("Unknown error: {0}")]
    Other(String),
}

impl From<OidcError> for TwitchApiClientError {
    fn from(err: OidcError) -> Self {
        Self::OidcError(format!("{}", err))
    }
}

impl<RE: Error + 'static> From<DiscoveryError<RE>> for OidcError {
    fn from(err: DiscoveryError<RE>) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl<RE: Error + 'static, T: ErrorResponse + 'static> From<RequestTokenError<RE, T>> for OidcError {
    fn from(err: RequestTokenError<RE, T>) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl From<ClaimsVerificationError> for OidcError {
    fn from(err: ClaimsVerificationError) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl From<SigningError> for OidcError {
    fn from(err: SigningError) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl From<ConfigurationError> for OidcError {
    fn from(err: ConfigurationError) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl<RE: Error + 'static> From<UserInfoError<RE>> for OidcError {
    fn from(err: UserInfoError<RE>) -> Self {
        Self::StandardOidc(format!("{}", err))
    }
}

impl From<ParseError> for TwitchApiClientError {
    fn from(err: ParseError) -> Self {
        Self::UrlParseError(format!("{}", err))
    }
}
