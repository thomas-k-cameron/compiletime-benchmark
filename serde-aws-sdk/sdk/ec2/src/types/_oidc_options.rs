// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Options for OIDC-based, user-identity type trust provider.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct OidcOptions {
    /// <p>The OIDC issuer.</p>
    #[doc(hidden)]
    pub issuer: std::option::Option<std::string::String>,
    /// <p>The OIDC authorization endpoint.</p>
    #[doc(hidden)]
    pub authorization_endpoint: std::option::Option<std::string::String>,
    /// <p>The OIDC token endpoint.</p>
    #[doc(hidden)]
    pub token_endpoint: std::option::Option<std::string::String>,
    /// <p>The OIDC user info endpoint.</p>
    #[doc(hidden)]
    pub user_info_endpoint: std::option::Option<std::string::String>,
    /// <p>The client identifier.</p>
    #[doc(hidden)]
    pub client_id: std::option::Option<std::string::String>,
    /// <p>The client secret.</p>
    #[doc(hidden)]
    pub client_secret: std::option::Option<std::string::String>,
    /// <p>The OpenID Connect (OIDC) scope specified.</p>
    #[doc(hidden)]
    pub scope: std::option::Option<std::string::String>,
}
impl OidcOptions {
    /// <p>The OIDC issuer.</p>
    pub fn issuer(&self) -> std::option::Option<&str> {
        self.issuer.as_deref()
    }
    /// <p>The OIDC authorization endpoint.</p>
    pub fn authorization_endpoint(&self) -> std::option::Option<&str> {
        self.authorization_endpoint.as_deref()
    }
    /// <p>The OIDC token endpoint.</p>
    pub fn token_endpoint(&self) -> std::option::Option<&str> {
        self.token_endpoint.as_deref()
    }
    /// <p>The OIDC user info endpoint.</p>
    pub fn user_info_endpoint(&self) -> std::option::Option<&str> {
        self.user_info_endpoint.as_deref()
    }
    /// <p>The client identifier.</p>
    pub fn client_id(&self) -> std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>The client secret.</p>
    pub fn client_secret(&self) -> std::option::Option<&str> {
        self.client_secret.as_deref()
    }
    /// <p>The OpenID Connect (OIDC) scope specified.</p>
    pub fn scope(&self) -> std::option::Option<&str> {
        self.scope.as_deref()
    }
}
impl OidcOptions {
    /// Creates a new builder-style object to manufacture [`OidcOptions`](crate::types::OidcOptions).
    pub fn builder() -> crate::types::builders::OidcOptionsBuilder {
        crate::types::builders::OidcOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::OidcOptions;
/// A builder for [`OidcOptions`](crate::types::OidcOptions).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct OidcOptionsBuilder {
    pub(crate) issuer: std::option::Option<std::string::String>,
    pub(crate) authorization_endpoint: std::option::Option<std::string::String>,
    pub(crate) token_endpoint: std::option::Option<std::string::String>,
    pub(crate) user_info_endpoint: std::option::Option<std::string::String>,
    pub(crate) client_id: std::option::Option<std::string::String>,
    pub(crate) client_secret: std::option::Option<std::string::String>,
    pub(crate) scope: std::option::Option<std::string::String>,
}
impl OidcOptionsBuilder {
    /// <p>The OIDC issuer.</p>
    pub fn issuer(mut self, input: impl Into<std::string::String>) -> Self {
        self.issuer = Some(input.into());
        self
    }
    /// <p>The OIDC issuer.</p>
    pub fn set_issuer(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.issuer = input;
        self
    }
    /// <p>The OIDC authorization endpoint.</p>
    pub fn authorization_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
        self.authorization_endpoint = Some(input.into());
        self
    }
    /// <p>The OIDC authorization endpoint.</p>
    pub fn set_authorization_endpoint(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.authorization_endpoint = input;
        self
    }
    /// <p>The OIDC token endpoint.</p>
    pub fn token_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
        self.token_endpoint = Some(input.into());
        self
    }
    /// <p>The OIDC token endpoint.</p>
    pub fn set_token_endpoint(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.token_endpoint = input;
        self
    }
    /// <p>The OIDC user info endpoint.</p>
    pub fn user_info_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_info_endpoint = Some(input.into());
        self
    }
    /// <p>The OIDC user info endpoint.</p>
    pub fn set_user_info_endpoint(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.user_info_endpoint = input;
        self
    }
    /// <p>The client identifier.</p>
    pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_id = Some(input.into());
        self
    }
    /// <p>The client identifier.</p>
    pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>The client secret.</p>
    pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_secret = Some(input.into());
        self
    }
    /// <p>The client secret.</p>
    pub fn set_client_secret(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_secret = input;
        self
    }
    /// <p>The OpenID Connect (OIDC) scope specified.</p>
    pub fn scope(mut self, input: impl Into<std::string::String>) -> Self {
        self.scope = Some(input.into());
        self
    }
    /// <p>The OpenID Connect (OIDC) scope specified.</p>
    pub fn set_scope(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.scope = input;
        self
    }
    /// Consumes the builder and constructs a [`OidcOptions`](crate::types::OidcOptions).
    pub fn build(self) -> crate::types::OidcOptions {
        crate::types::OidcOptions {
            issuer: self.issuer,
            authorization_endpoint: self.authorization_endpoint,
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            scope: self.scope,
        }
    }
}
