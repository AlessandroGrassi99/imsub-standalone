use serde::{Deserialize, Serialize};
use std::time::Duration;

use openidconnect::{
    AccessToken, AdditionalClaims, Client, EmptyAdditionalClaims, ExtraTokenFields, GenderClaim,
    IdToken, IdTokenFields, JsonWebKeyType, JweContentEncryptionAlgorithm, JwsSigningAlgorithm,
    OAuth2TokenResponse, RefreshToken, Scope, StandardErrorResponse, TokenResponse, TokenType,
};

use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreIdTokenFields,
    CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm, CoreRevocableToken, CoreRevocationErrorResponse,
    CoreTokenIntrospectionResponse, CoreTokenType,
};

use oauth2::helpers;

pub type CoreTwitchTokenResponse = TwitchTokenResponse<CoreIdTokenFields, CoreTokenType>;

pub type CoreTwitchOidcClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    CoreTwitchTokenResponse,
    CoreTokenType,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TwitchTokenResponse<EF, TT>
where
    EF: ExtraTokenFields,
    TT: TokenType,
{
    access_token: AccessToken,
    #[serde(bound = "TT: TokenType")]
    #[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
    token_type: TT,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,
    #[serde(rename = "scope")]
    //  #[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
    #[serde(serialize_with = "helpers::serialize_space_delimited_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    scopes: Option<Vec<Scope>>,

    #[serde(bound = "EF: ExtraTokenFields")]
    #[serde(flatten)]
    extra_fields: EF,
}

impl<EF, TT> TwitchTokenResponse<EF, TT>
where
    EF: ExtraTokenFields,
    TT: TokenType,
{
    ///
    /// Instantiate a new OAuth2 token response.
    ///
    #[allow(dead_code)]
    pub fn new(access_token: AccessToken, token_type: TT, extra_fields: EF) -> Self {
        Self {
            access_token,
            token_type,
            expires_in: None,
            refresh_token: None,
            scopes: None,
            extra_fields,
        }
    }

    ///
    /// Set the `access_token` field.
    ///
    #[allow(dead_code)]
    pub fn set_access_token(&mut self, access_token: AccessToken) {
        self.access_token = access_token;
    }

    ///
    /// Set the `token_type` field.
    ///
    #[allow(dead_code)]
    pub fn set_token_type(&mut self, token_type: TT) {
        self.token_type = token_type;
    }

    ///
    /// Set the `expires_in` field.
    ///
    #[allow(dead_code)]
    pub fn set_expires_in(&mut self, expires_in: Option<&Duration>) {
        self.expires_in = expires_in.map(Duration::as_secs);
    }

    ///
    /// Set the `refresh_token` field.
    ///
    #[allow(dead_code)]
    pub fn set_refresh_token(&mut self, refresh_token: Option<RefreshToken>) {
        self.refresh_token = refresh_token;
    }

    ///
    /// Set the `scopes` field.
    ///
    #[allow(dead_code)]
    pub fn set_scopes(&mut self, scopes: Option<Vec<Scope>>) {
        self.scopes = scopes;
    }

    ///
    /// Extra fields defined by the client application.
    ///
    #[allow(dead_code)]
    pub fn extra_fields(&self) -> &EF {
        &self.extra_fields
    }

    ///
    /// Set the extra fields defined by the client application.
    ///
    #[allow(dead_code)]
    pub fn set_extra_fields(&mut self, extra_fields: EF) {
        self.extra_fields = extra_fields;
    }
}

impl<AC, EF, GC, JE, JS, JT, TT> OAuth2TokenResponse<TT>
    for TwitchTokenResponse<IdTokenFields<AC, EF, GC, JE, JS, JT>, TT>
where
    AC: AdditionalClaims,
    EF: ExtraTokenFields,
    GC: GenderClaim,
    JE: JweContentEncryptionAlgorithm<JT>,
    JS: JwsSigningAlgorithm<JT>,
    JT: JsonWebKeyType,
    TT: TokenType,
{
    ///
    /// REQUIRED. The access token issued by the authorization server.
    ///
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    ///
    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    ///
    fn token_type(&self) -> &TT {
        &self.token_type
    }
    ///
    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    ///
    fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }
    ///
    /// OPTIONAL. The refresh token, which can be used to obtain new access tokens using the same
    /// authorization grant as described in
    /// [Section 6](https://tools.ietf.org/html/rfc6749#section-6).
    ///
    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }
    ///
    /// OPTIONAL, if identical to the scope requested by the client; otherwise, REQUIRED. The
    /// scope of the access token as described by
    /// [Section 3.3](https://tools.ietf.org/html/rfc6749#section-3.3). If included in the response,
    /// this space-delimited field is parsed into a `Vec` of individual scopes. If omitted from
    /// the response, this field is `None`.
    ///
    fn scopes(&self) -> Option<&Vec<Scope>> {
        self.scopes.as_ref()
    }
}

impl<AC, EF, GC, JE, JS, JT, TT> TokenResponse<AC, GC, JE, JS, JT, TT>
    for TwitchTokenResponse<IdTokenFields<AC, EF, GC, JE, JS, JT>, TT>
where
    AC: AdditionalClaims,
    EF: ExtraTokenFields,
    GC: GenderClaim,
    JE: JweContentEncryptionAlgorithm<JT>,
    JS: JwsSigningAlgorithm<JT>,
    JT: JsonWebKeyType,
    TT: TokenType,
{
    fn id_token(&self) -> Option<&IdToken<AC, GC, JE, JS, JT>> {
        self.extra_fields().id_token()
    }
}
