// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct ModifyVerifiedAccessTrustProviderInput {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    #[doc(hidden)]
    pub verified_access_trust_provider_id: std::option::Option<std::string::String>,
    /// <p>The OpenID Connect details for an <code>oidc</code>-type, user-identity based trust provider.</p>
    #[doc(hidden)]
    pub oidc_options:
        std::option::Option<crate::types::ModifyVerifiedAccessTrustProviderOidcOptions>,
    /// <p>A description for the Amazon Web Services Verified Access trust provider.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
}
impl ModifyVerifiedAccessTrustProviderInput {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn verified_access_trust_provider_id(&self) -> std::option::Option<&str> {
        self.verified_access_trust_provider_id.as_deref()
    }
    /// <p>The OpenID Connect details for an <code>oidc</code>-type, user-identity based trust provider.</p>
    pub fn oidc_options(
        &self,
    ) -> std::option::Option<&crate::types::ModifyVerifiedAccessTrustProviderOidcOptions> {
        self.oidc_options.as_ref()
    }
    /// <p>A description for the Amazon Web Services Verified Access trust provider.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl ModifyVerifiedAccessTrustProviderInput {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessTrustProviderInput`](crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput).
    pub fn builder() -> crate::operation::modify_verified_access_trust_provider::builders::ModifyVerifiedAccessTrustProviderInputBuilder{
        crate::operation::modify_verified_access_trust_provider::builders::ModifyVerifiedAccessTrustProviderInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput;
/// A builder for [`ModifyVerifiedAccessTrustProviderInput`](crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput).
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
pub struct ModifyVerifiedAccessTrustProviderInputBuilder {
    pub(crate) verified_access_trust_provider_id: std::option::Option<std::string::String>,
    pub(crate) oidc_options:
        std::option::Option<crate::types::ModifyVerifiedAccessTrustProviderOidcOptions>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) client_token: std::option::Option<std::string::String>,
}
impl ModifyVerifiedAccessTrustProviderInputBuilder {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn verified_access_trust_provider_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn set_verified_access_trust_provider_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.verified_access_trust_provider_id = input;
        self
    }
    /// <p>The OpenID Connect details for an <code>oidc</code>-type, user-identity based trust provider.</p>
    pub fn oidc_options(
        mut self,
        input: crate::types::ModifyVerifiedAccessTrustProviderOidcOptions,
    ) -> Self {
        self.oidc_options = Some(input);
        self
    }
    /// <p>The OpenID Connect details for an <code>oidc</code>-type, user-identity based trust provider.</p>
    pub fn set_oidc_options(
        mut self,
        input: std::option::Option<crate::types::ModifyVerifiedAccessTrustProviderOidcOptions>,
    ) -> Self {
        self.oidc_options = input;
        self
    }
    /// <p>A description for the Amazon Web Services Verified Access trust provider.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description for the Amazon Web Services Verified Access trust provider.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessTrustProviderInput`](crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput).
    pub fn build(self) -> Result<crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput {
                verified_access_trust_provider_id: self.verified_access_trust_provider_id
                ,
                oidc_options: self.oidc_options
                ,
                description: self.description
                ,
                dry_run: self.dry_run
                ,
                client_token: self.client_token
                ,
            }
        )
    }
}
