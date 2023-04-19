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
pub struct ResetServiceSpecificCredentialOutput {
    /// <p>A structure with details about the updated service-specific credential, including the new password.</p> <important>
    /// <p>This is the <b>only</b> time that you can access the password. You cannot recover the password later, but you can reset it again.</p>
    /// </important>
    #[doc(hidden)]
    pub service_specific_credential: std::option::Option<crate::types::ServiceSpecificCredential>,
    _request_id: Option<String>,
}
impl ResetServiceSpecificCredentialOutput {
    /// <p>A structure with details about the updated service-specific credential, including the new password.</p> <important>
    /// <p>This is the <b>only</b> time that you can access the password. You cannot recover the password later, but you can reset it again.</p>
    /// </important>
    pub fn service_specific_credential(
        &self,
    ) -> std::option::Option<&crate::types::ServiceSpecificCredential> {
        self.service_specific_credential.as_ref()
    }
}
impl aws_http::request_id::RequestId for ResetServiceSpecificCredentialOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ResetServiceSpecificCredentialOutput {
    /// Creates a new builder-style object to manufacture [`ResetServiceSpecificCredentialOutput`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput).
    pub fn builder() -> crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialOutputBuilder{
        crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput;
/// A builder for [`ResetServiceSpecificCredentialOutput`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput).
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
pub struct ResetServiceSpecificCredentialOutputBuilder {
    pub(crate) service_specific_credential:
        std::option::Option<crate::types::ServiceSpecificCredential>,
    _request_id: Option<String>,
}
impl ResetServiceSpecificCredentialOutputBuilder {
    /// <p>A structure with details about the updated service-specific credential, including the new password.</p> <important>
    /// <p>This is the <b>only</b> time that you can access the password. You cannot recover the password later, but you can reset it again.</p>
    /// </important>
    pub fn service_specific_credential(
        mut self,
        input: crate::types::ServiceSpecificCredential,
    ) -> Self {
        self.service_specific_credential = Some(input);
        self
    }
    /// <p>A structure with details about the updated service-specific credential, including the new password.</p> <important>
    /// <p>This is the <b>only</b> time that you can access the password. You cannot recover the password later, but you can reset it again.</p>
    /// </important>
    pub fn set_service_specific_credential(
        mut self,
        input: std::option::Option<crate::types::ServiceSpecificCredential>,
    ) -> Self {
        self.service_specific_credential = input;
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
    /// Consumes the builder and constructs a [`ResetServiceSpecificCredentialOutput`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput).
    pub fn build(
        self,
    ) -> crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput
    {
        crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput {
            service_specific_credential: self.service_specific_credential,
            _request_id: self._request_id,
        }
    }
}
