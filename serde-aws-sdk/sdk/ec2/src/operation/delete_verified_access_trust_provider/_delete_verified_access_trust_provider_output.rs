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
pub struct DeleteVerifiedAccessTrustProviderOutput {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    #[doc(hidden)]
    pub verified_access_trust_provider:
        std::option::Option<crate::types::VerifiedAccessTrustProvider>,
    _request_id: Option<String>,
}
impl DeleteVerifiedAccessTrustProviderOutput {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn verified_access_trust_provider(
        &self,
    ) -> std::option::Option<&crate::types::VerifiedAccessTrustProvider> {
        self.verified_access_trust_provider.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteVerifiedAccessTrustProviderOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteVerifiedAccessTrustProviderOutput {
    /// Creates a new builder-style object to manufacture [`DeleteVerifiedAccessTrustProviderOutput`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput).
    pub fn builder() -> crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderOutputBuilder{
        crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput;
/// A builder for [`DeleteVerifiedAccessTrustProviderOutput`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput).
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
pub struct DeleteVerifiedAccessTrustProviderOutputBuilder {
    pub(crate) verified_access_trust_provider:
        std::option::Option<crate::types::VerifiedAccessTrustProvider>,
    _request_id: Option<String>,
}
impl DeleteVerifiedAccessTrustProviderOutputBuilder {
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn verified_access_trust_provider(
        mut self,
        input: crate::types::VerifiedAccessTrustProvider,
    ) -> Self {
        self.verified_access_trust_provider = Some(input);
        self
    }
    /// <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    pub fn set_verified_access_trust_provider(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessTrustProvider>,
    ) -> Self {
        self.verified_access_trust_provider = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVerifiedAccessTrustProviderOutput`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput).
    pub fn build(self) -> crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput{
        crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput {
            verified_access_trust_provider: self.verified_access_trust_provider
            ,
            _request_id: self._request_id,
        }
    }
}
