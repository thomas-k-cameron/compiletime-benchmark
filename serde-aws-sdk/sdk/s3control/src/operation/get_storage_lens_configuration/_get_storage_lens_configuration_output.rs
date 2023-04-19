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
pub struct GetStorageLensConfigurationOutput {
    /// <p>The S3 Storage Lens configuration requested.</p>
    #[doc(hidden)]
    pub storage_lens_configuration: std::option::Option<crate::types::StorageLensConfiguration>,
    _request_id: Option<String>,
}
impl GetStorageLensConfigurationOutput {
    /// <p>The S3 Storage Lens configuration requested.</p>
    pub fn storage_lens_configuration(
        &self,
    ) -> std::option::Option<&crate::types::StorageLensConfiguration> {
        self.storage_lens_configuration.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetStorageLensConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetStorageLensConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetStorageLensConfigurationOutput`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput).
    pub fn builder() -> crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationOutputBuilder{
        crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput;
/// A builder for [`GetStorageLensConfigurationOutput`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput).
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
pub struct GetStorageLensConfigurationOutputBuilder {
    pub(crate) storage_lens_configuration:
        std::option::Option<crate::types::StorageLensConfiguration>,
    _request_id: Option<String>,
}
impl GetStorageLensConfigurationOutputBuilder {
    /// <p>The S3 Storage Lens configuration requested.</p>
    pub fn storage_lens_configuration(
        mut self,
        input: crate::types::StorageLensConfiguration,
    ) -> Self {
        self.storage_lens_configuration = Some(input);
        self
    }
    /// <p>The S3 Storage Lens configuration requested.</p>
    pub fn set_storage_lens_configuration(
        mut self,
        input: std::option::Option<crate::types::StorageLensConfiguration>,
    ) -> Self {
        self.storage_lens_configuration = input;
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
    /// Consumes the builder and constructs a [`GetStorageLensConfigurationOutput`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput {
        crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput {
            storage_lens_configuration: self.storage_lens_configuration,
            _request_id: self._request_id,
        }
    }
}
