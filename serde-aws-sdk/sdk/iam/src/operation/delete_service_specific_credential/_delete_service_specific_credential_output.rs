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
pub struct DeleteServiceSpecificCredentialOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteServiceSpecificCredentialOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteServiceSpecificCredentialOutput {
    /// Creates a new builder-style object to manufacture [`DeleteServiceSpecificCredentialOutput`](crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput).
    pub fn builder() -> crate::operation::delete_service_specific_credential::builders::DeleteServiceSpecificCredentialOutputBuilder{
        crate::operation::delete_service_specific_credential::builders::DeleteServiceSpecificCredentialOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput;
/// A builder for [`DeleteServiceSpecificCredentialOutput`](crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput).
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
pub struct DeleteServiceSpecificCredentialOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteServiceSpecificCredentialOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteServiceSpecificCredentialOutput`](crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput
    {
        crate::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput {
            _request_id: self._request_id,
        }
    }
}