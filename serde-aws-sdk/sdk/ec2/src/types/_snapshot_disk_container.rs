// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The disk container object for the import snapshot request.</p>
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
pub struct SnapshotDiskContainer {
    /// <p>The description of the disk image being imported.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The format of the disk image being imported.</p>
    /// <p>Valid values: <code>VHD</code> | <code>VMDK</code> | <code>RAW</code> </p>
    #[doc(hidden)]
    pub format: std::option::Option<std::string::String>,
    /// <p>The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..).</p>
    #[doc(hidden)]
    pub url: std::option::Option<std::string::String>,
    /// <p>The Amazon S3 bucket for the disk image.</p>
    #[doc(hidden)]
    pub user_bucket: std::option::Option<crate::types::UserBucket>,
}
impl SnapshotDiskContainer {
    /// <p>The description of the disk image being imported.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The format of the disk image being imported.</p>
    /// <p>Valid values: <code>VHD</code> | <code>VMDK</code> | <code>RAW</code> </p>
    pub fn format(&self) -> std::option::Option<&str> {
        self.format.as_deref()
    }
    /// <p>The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..).</p>
    pub fn url(&self) -> std::option::Option<&str> {
        self.url.as_deref()
    }
    /// <p>The Amazon S3 bucket for the disk image.</p>
    pub fn user_bucket(&self) -> std::option::Option<&crate::types::UserBucket> {
        self.user_bucket.as_ref()
    }
}
impl SnapshotDiskContainer {
    /// Creates a new builder-style object to manufacture [`SnapshotDiskContainer`](crate::types::SnapshotDiskContainer).
    pub fn builder() -> crate::types::builders::SnapshotDiskContainerBuilder {
        crate::types::builders::SnapshotDiskContainerBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SnapshotDiskContainer;
/// A builder for [`SnapshotDiskContainer`](crate::types::SnapshotDiskContainer).
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
pub struct SnapshotDiskContainerBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) format: std::option::Option<std::string::String>,
    pub(crate) url: std::option::Option<std::string::String>,
    pub(crate) user_bucket: std::option::Option<crate::types::UserBucket>,
}
impl SnapshotDiskContainerBuilder {
    /// <p>The description of the disk image being imported.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of the disk image being imported.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The format of the disk image being imported.</p>
    /// <p>Valid values: <code>VHD</code> | <code>VMDK</code> | <code>RAW</code> </p>
    pub fn format(mut self, input: impl Into<std::string::String>) -> Self {
        self.format = Some(input.into());
        self
    }
    /// <p>The format of the disk image being imported.</p>
    /// <p>Valid values: <code>VHD</code> | <code>VMDK</code> | <code>RAW</code> </p>
    pub fn set_format(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.format = input;
        self
    }
    /// <p>The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..).</p>
    pub fn url(mut self, input: impl Into<std::string::String>) -> Self {
        self.url = Some(input.into());
        self
    }
    /// <p>The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..).</p>
    pub fn set_url(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.url = input;
        self
    }
    /// <p>The Amazon S3 bucket for the disk image.</p>
    pub fn user_bucket(mut self, input: crate::types::UserBucket) -> Self {
        self.user_bucket = Some(input);
        self
    }
    /// <p>The Amazon S3 bucket for the disk image.</p>
    pub fn set_user_bucket(mut self, input: std::option::Option<crate::types::UserBucket>) -> Self {
        self.user_bucket = input;
        self
    }
    /// Consumes the builder and constructs a [`SnapshotDiskContainer`](crate::types::SnapshotDiskContainer).
    pub fn build(self) -> crate::types::SnapshotDiskContainer {
        crate::types::SnapshotDiskContainer {
            description: self.description,
            format: self.format,
            url: self.url,
            user_bucket: self.user_bucket,
        }
    }
}
