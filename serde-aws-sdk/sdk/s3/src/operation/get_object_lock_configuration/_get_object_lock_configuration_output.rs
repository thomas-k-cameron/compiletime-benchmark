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
pub struct GetObjectLockConfigurationOutput {
    /// <p>The specified bucket's Object Lock configuration.</p>
    #[doc(hidden)]
    pub object_lock_configuration: std::option::Option<crate::types::ObjectLockConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectLockConfigurationOutput {
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub fn object_lock_configuration(
        &self,
    ) -> std::option::Option<&crate::types::ObjectLockConfiguration> {
        self.object_lock_configuration.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetObjectLockConfigurationOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetObjectLockConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetObjectLockConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectLockConfigurationOutput`](crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput).
    pub fn builder() -> crate::operation::get_object_lock_configuration::builders::GetObjectLockConfigurationOutputBuilder{
        crate::operation::get_object_lock_configuration::builders::GetObjectLockConfigurationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput;
/// A builder for [`GetObjectLockConfigurationOutput`](crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput).
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
pub struct GetObjectLockConfigurationOutputBuilder {
    pub(crate) object_lock_configuration:
        std::option::Option<crate::types::ObjectLockConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectLockConfigurationOutputBuilder {
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub fn object_lock_configuration(
        mut self,
        input: crate::types::ObjectLockConfiguration,
    ) -> Self {
        self.object_lock_configuration = Some(input);
        self
    }
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub fn set_object_lock_configuration(
        mut self,
        input: std::option::Option<crate::types::ObjectLockConfiguration>,
    ) -> Self {
        self.object_lock_configuration = input;
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
    /// Consumes the builder and constructs a [`GetObjectLockConfigurationOutput`](crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput {
        crate::operation::get_object_lock_configuration::GetObjectLockConfigurationOutput {
            object_lock_configuration: self.object_lock_configuration,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}