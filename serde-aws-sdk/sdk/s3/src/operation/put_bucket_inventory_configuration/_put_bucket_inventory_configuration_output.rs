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
pub struct PutBucketInventoryConfigurationOutput {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl crate::s3_request_id::RequestIdExt for PutBucketInventoryConfigurationOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for PutBucketInventoryConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutBucketInventoryConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`PutBucketInventoryConfigurationOutput`](crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput).
    pub fn builder() -> crate::operation::put_bucket_inventory_configuration::builders::PutBucketInventoryConfigurationOutputBuilder{
        crate::operation::put_bucket_inventory_configuration::builders::PutBucketInventoryConfigurationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput;
/// A builder for [`PutBucketInventoryConfigurationOutput`](crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput).
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
pub struct PutBucketInventoryConfigurationOutputBuilder {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutBucketInventoryConfigurationOutputBuilder {
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
    /// Consumes the builder and constructs a [`PutBucketInventoryConfigurationOutput`](crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput).
    pub fn build(
        self,
    ) -> crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput
    {
        crate::operation::put_bucket_inventory_configuration::PutBucketInventoryConfigurationOutput {
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
