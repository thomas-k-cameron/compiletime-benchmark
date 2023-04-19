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
pub struct GetStorageLensConfigurationTaggingOutput {
    /// <p>The tags of S3 Storage Lens configuration requested.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::StorageLensTag>>,
    _request_id: Option<String>,
}
impl GetStorageLensConfigurationTaggingOutput {
    /// <p>The tags of S3 Storage Lens configuration requested.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::StorageLensTag]> {
        self.tags.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetStorageLensConfigurationTaggingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetStorageLensConfigurationTaggingOutput {
    /// Creates a new builder-style object to manufacture [`GetStorageLensConfigurationTaggingOutput`](crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput).
    pub fn builder() -> crate::operation::get_storage_lens_configuration_tagging::builders::GetStorageLensConfigurationTaggingOutputBuilder{
        crate::operation::get_storage_lens_configuration_tagging::builders::GetStorageLensConfigurationTaggingOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput;
/// A builder for [`GetStorageLensConfigurationTaggingOutput`](crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput).
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
pub struct GetStorageLensConfigurationTaggingOutputBuilder {
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::StorageLensTag>>,
    _request_id: Option<String>,
}
impl GetStorageLensConfigurationTaggingOutputBuilder {
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags of S3 Storage Lens configuration requested.</p>
    pub fn tags(mut self, input: crate::types::StorageLensTag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags of S3 Storage Lens configuration requested.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::StorageLensTag>>,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`GetStorageLensConfigurationTaggingOutput`](crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput).
    pub fn build(self) -> crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput{
        crate::operation::get_storage_lens_configuration_tagging::GetStorageLensConfigurationTaggingOutput {
            tags: self.tags
            ,
            _request_id: self._request_id,
        }
    }
}
