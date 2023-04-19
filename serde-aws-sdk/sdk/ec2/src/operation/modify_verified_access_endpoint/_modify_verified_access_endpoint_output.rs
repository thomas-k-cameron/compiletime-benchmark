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
pub struct ModifyVerifiedAccessEndpointOutput {
    /// <p>The Amazon Web Services Verified Access endpoint details.</p>
    #[doc(hidden)]
    pub verified_access_endpoint: std::option::Option<crate::types::VerifiedAccessEndpoint>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessEndpointOutput {
    /// <p>The Amazon Web Services Verified Access endpoint details.</p>
    pub fn verified_access_endpoint(
        &self,
    ) -> std::option::Option<&crate::types::VerifiedAccessEndpoint> {
        self.verified_access_endpoint.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyVerifiedAccessEndpointOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVerifiedAccessEndpointOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessEndpointOutput`](crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput).
    pub fn builder() -> crate::operation::modify_verified_access_endpoint::builders::ModifyVerifiedAccessEndpointOutputBuilder{
        crate::operation::modify_verified_access_endpoint::builders::ModifyVerifiedAccessEndpointOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput;
/// A builder for [`ModifyVerifiedAccessEndpointOutput`](crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput).
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
pub struct ModifyVerifiedAccessEndpointOutputBuilder {
    pub(crate) verified_access_endpoint: std::option::Option<crate::types::VerifiedAccessEndpoint>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessEndpointOutputBuilder {
    /// <p>The Amazon Web Services Verified Access endpoint details.</p>
    pub fn verified_access_endpoint(mut self, input: crate::types::VerifiedAccessEndpoint) -> Self {
        self.verified_access_endpoint = Some(input);
        self
    }
    /// <p>The Amazon Web Services Verified Access endpoint details.</p>
    pub fn set_verified_access_endpoint(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessEndpoint>,
    ) -> Self {
        self.verified_access_endpoint = input;
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
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessEndpointOutput`](crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput {
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput {
            verified_access_endpoint: self.verified_access_endpoint,
            _request_id: self._request_id,
        }
    }
}
