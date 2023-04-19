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
pub struct DeleteBucketAnalyticsConfigurationOutput {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl crate::s3_request_id::RequestIdExt for DeleteBucketAnalyticsConfigurationOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for DeleteBucketAnalyticsConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteBucketAnalyticsConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketAnalyticsConfigurationOutput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput).
    pub fn builder() -> crate::operation::delete_bucket_analytics_configuration::builders::DeleteBucketAnalyticsConfigurationOutputBuilder{
        crate::operation::delete_bucket_analytics_configuration::builders::DeleteBucketAnalyticsConfigurationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput;
/// A builder for [`DeleteBucketAnalyticsConfigurationOutput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput).
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
pub struct DeleteBucketAnalyticsConfigurationOutputBuilder {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl DeleteBucketAnalyticsConfigurationOutputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteBucketAnalyticsConfigurationOutput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput).
    pub fn build(self) -> crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput{
        crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationOutput {
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}