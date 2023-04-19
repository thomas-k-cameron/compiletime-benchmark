// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an import volume task.</p>
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
pub struct ImportVolumeTaskDetails {
    /// <p>The Availability Zone where the resulting volume will reside.</p>
    #[doc(hidden)]
    pub availability_zone: std::option::Option<std::string::String>,
    /// <p>The number of bytes converted so far.</p>
    #[doc(hidden)]
    pub bytes_converted: std::option::Option<i64>,
    /// <p>The description you provided when starting the import volume task.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The image.</p>
    #[doc(hidden)]
    pub image: std::option::Option<crate::types::DiskImageDescription>,
    /// <p>The volume.</p>
    #[doc(hidden)]
    pub volume: std::option::Option<crate::types::DiskImageVolumeDescription>,
}
impl ImportVolumeTaskDetails {
    /// <p>The Availability Zone where the resulting volume will reside.</p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The number of bytes converted so far.</p>
    pub fn bytes_converted(&self) -> std::option::Option<i64> {
        self.bytes_converted
    }
    /// <p>The description you provided when starting the import volume task.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The image.</p>
    pub fn image(&self) -> std::option::Option<&crate::types::DiskImageDescription> {
        self.image.as_ref()
    }
    /// <p>The volume.</p>
    pub fn volume(&self) -> std::option::Option<&crate::types::DiskImageVolumeDescription> {
        self.volume.as_ref()
    }
}
impl ImportVolumeTaskDetails {
    /// Creates a new builder-style object to manufacture [`ImportVolumeTaskDetails`](crate::types::ImportVolumeTaskDetails).
    pub fn builder() -> crate::types::builders::ImportVolumeTaskDetailsBuilder {
        crate::types::builders::ImportVolumeTaskDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ImportVolumeTaskDetails;
/// A builder for [`ImportVolumeTaskDetails`](crate::types::ImportVolumeTaskDetails).
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
pub struct ImportVolumeTaskDetailsBuilder {
    pub(crate) availability_zone: std::option::Option<std::string::String>,
    pub(crate) bytes_converted: std::option::Option<i64>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) image: std::option::Option<crate::types::DiskImageDescription>,
    pub(crate) volume: std::option::Option<crate::types::DiskImageVolumeDescription>,
}
impl ImportVolumeTaskDetailsBuilder {
    /// <p>The Availability Zone where the resulting volume will reside.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.availability_zone = Some(input.into());
        self
    }
    /// <p>The Availability Zone where the resulting volume will reside.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The number of bytes converted so far.</p>
    pub fn bytes_converted(mut self, input: i64) -> Self {
        self.bytes_converted = Some(input);
        self
    }
    /// <p>The number of bytes converted so far.</p>
    pub fn set_bytes_converted(mut self, input: std::option::Option<i64>) -> Self {
        self.bytes_converted = input;
        self
    }
    /// <p>The description you provided when starting the import volume task.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description you provided when starting the import volume task.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The image.</p>
    pub fn image(mut self, input: crate::types::DiskImageDescription) -> Self {
        self.image = Some(input);
        self
    }
    /// <p>The image.</p>
    pub fn set_image(
        mut self,
        input: std::option::Option<crate::types::DiskImageDescription>,
    ) -> Self {
        self.image = input;
        self
    }
    /// <p>The volume.</p>
    pub fn volume(mut self, input: crate::types::DiskImageVolumeDescription) -> Self {
        self.volume = Some(input);
        self
    }
    /// <p>The volume.</p>
    pub fn set_volume(
        mut self,
        input: std::option::Option<crate::types::DiskImageVolumeDescription>,
    ) -> Self {
        self.volume = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportVolumeTaskDetails`](crate::types::ImportVolumeTaskDetails).
    pub fn build(self) -> crate::types::ImportVolumeTaskDetails {
        crate::types::ImportVolumeTaskDetails {
            availability_zone: self.availability_zone,
            bytes_converted: self.bytes_converted,
            description: self.description,
            image: self.image,
            volume: self.volume,
        }
    }
}
