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
pub struct GetObjectRetentionOutput {
    /// <p>The container element for an object's retention settings.</p>
    #[doc(hidden)]
    pub retention: std::option::Option<crate::types::ObjectLockRetention>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectRetentionOutput {
    /// <p>The container element for an object's retention settings.</p>
    pub fn retention(&self) -> std::option::Option<&crate::types::ObjectLockRetention> {
        self.retention.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetObjectRetentionOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetObjectRetentionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetObjectRetentionOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectRetentionOutput`](crate::operation::get_object_retention::GetObjectRetentionOutput).
    pub fn builder(
    ) -> crate::operation::get_object_retention::builders::GetObjectRetentionOutputBuilder {
        crate::operation::get_object_retention::builders::GetObjectRetentionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_object_retention::GetObjectRetentionOutput;
/// A builder for [`GetObjectRetentionOutput`](crate::operation::get_object_retention::GetObjectRetentionOutput).
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
pub struct GetObjectRetentionOutputBuilder {
    pub(crate) retention: std::option::Option<crate::types::ObjectLockRetention>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectRetentionOutputBuilder {
    /// <p>The container element for an object's retention settings.</p>
    pub fn retention(mut self, input: crate::types::ObjectLockRetention) -> Self {
        self.retention = Some(input);
        self
    }
    /// <p>The container element for an object's retention settings.</p>
    pub fn set_retention(
        mut self,
        input: std::option::Option<crate::types::ObjectLockRetention>,
    ) -> Self {
        self.retention = input;
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
    /// Consumes the builder and constructs a [`GetObjectRetentionOutput`](crate::operation::get_object_retention::GetObjectRetentionOutput).
    pub fn build(self) -> crate::operation::get_object_retention::GetObjectRetentionOutput {
        crate::operation::get_object_retention::GetObjectRetentionOutput {
            retention: self.retention,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
