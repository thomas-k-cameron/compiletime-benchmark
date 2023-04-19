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
pub struct DeleteServiceOutput {
    /// <p>The full description of the deleted service.</p>
    #[doc(hidden)]
    pub service: std::option::Option<crate::types::Service>,
    _request_id: Option<String>,
}
impl DeleteServiceOutput {
    /// <p>The full description of the deleted service.</p>
    pub fn service(&self) -> std::option::Option<&crate::types::Service> {
        self.service.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteServiceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteServiceOutput {
    /// Creates a new builder-style object to manufacture [`DeleteServiceOutput`](crate::operation::delete_service::DeleteServiceOutput).
    pub fn builder() -> crate::operation::delete_service::builders::DeleteServiceOutputBuilder {
        crate::operation::delete_service::builders::DeleteServiceOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_service::DeleteServiceOutput;
/// A builder for [`DeleteServiceOutput`](crate::operation::delete_service::DeleteServiceOutput).
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
pub struct DeleteServiceOutputBuilder {
    pub(crate) service: std::option::Option<crate::types::Service>,
    _request_id: Option<String>,
}
impl DeleteServiceOutputBuilder {
    /// <p>The full description of the deleted service.</p>
    pub fn service(mut self, input: crate::types::Service) -> Self {
        self.service = Some(input);
        self
    }
    /// <p>The full description of the deleted service.</p>
    pub fn set_service(mut self, input: std::option::Option<crate::types::Service>) -> Self {
        self.service = input;
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
    /// Consumes the builder and constructs a [`DeleteServiceOutput`](crate::operation::delete_service::DeleteServiceOutput).
    pub fn build(self) -> crate::operation::delete_service::DeleteServiceOutput {
        crate::operation::delete_service::DeleteServiceOutput {
            service: self.service,
            _request_id: self._request_id,
        }
    }
}