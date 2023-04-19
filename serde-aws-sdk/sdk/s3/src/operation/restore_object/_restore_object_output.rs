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
pub struct RestoreObjectOutput {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    #[doc(hidden)]
    pub request_charged: std::option::Option<crate::types::RequestCharged>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    #[doc(hidden)]
    pub restore_output_path: std::option::Option<std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl RestoreObjectOutput {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(&self) -> std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub fn restore_output_path(&self) -> std::option::Option<&str> {
        self.restore_output_path.as_deref()
    }
}
impl crate::s3_request_id::RequestIdExt for RestoreObjectOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for RestoreObjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RestoreObjectOutput {
    /// Creates a new builder-style object to manufacture [`RestoreObjectOutput`](crate::operation::restore_object::RestoreObjectOutput).
    pub fn builder() -> crate::operation::restore_object::builders::RestoreObjectOutputBuilder {
        crate::operation::restore_object::builders::RestoreObjectOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::restore_object::RestoreObjectOutput;
/// A builder for [`RestoreObjectOutput`](crate::operation::restore_object::RestoreObjectOutput).
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
pub struct RestoreObjectOutputBuilder {
    pub(crate) request_charged: std::option::Option<crate::types::RequestCharged>,
    pub(crate) restore_output_path: std::option::Option<std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl RestoreObjectOutputBuilder {
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn set_request_charged(
        mut self,
        input: std::option::Option<crate::types::RequestCharged>,
    ) -> Self {
        self.request_charged = input;
        self
    }
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub fn restore_output_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.restore_output_path = Some(input.into());
        self
    }
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub fn set_restore_output_path(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.restore_output_path = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
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
    /// Consumes the builder and constructs a [`RestoreObjectOutput`](crate::operation::restore_object::RestoreObjectOutput).
    pub fn build(self) -> crate::operation::restore_object::RestoreObjectOutput {
        crate::operation::restore_object::RestoreObjectOutput {
            request_charged: self.request_charged,
            restore_output_path: self.restore_output_path,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
