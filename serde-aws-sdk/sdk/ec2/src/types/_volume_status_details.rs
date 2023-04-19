// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a volume status.</p>
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
pub struct VolumeStatusDetails {
    /// <p>The name of the volume status.</p>
    #[doc(hidden)]
    pub name: std::option::Option<crate::types::VolumeStatusName>,
    /// <p>The intended status of the volume status.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
}
impl VolumeStatusDetails {
    /// <p>The name of the volume status.</p>
    pub fn name(&self) -> std::option::Option<&crate::types::VolumeStatusName> {
        self.name.as_ref()
    }
    /// <p>The intended status of the volume status.</p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl VolumeStatusDetails {
    /// Creates a new builder-style object to manufacture [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
    pub fn builder() -> crate::types::builders::VolumeStatusDetailsBuilder {
        crate::types::builders::VolumeStatusDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VolumeStatusDetails;
/// A builder for [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
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
pub struct VolumeStatusDetailsBuilder {
    pub(crate) name: std::option::Option<crate::types::VolumeStatusName>,
    pub(crate) status: std::option::Option<std::string::String>,
}
impl VolumeStatusDetailsBuilder {
    /// <p>The name of the volume status.</p>
    pub fn name(mut self, input: crate::types::VolumeStatusName) -> Self {
        self.name = Some(input);
        self
    }
    /// <p>The name of the volume status.</p>
    pub fn set_name(mut self, input: std::option::Option<crate::types::VolumeStatusName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The intended status of the volume status.</p>
    pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
        self.status = Some(input.into());
        self
    }
    /// <p>The intended status of the volume status.</p>
    pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`VolumeStatusDetails`](crate::types::VolumeStatusDetails).
    pub fn build(self) -> crate::types::VolumeStatusDetails {
        crate::types::VolumeStatusDetails {
            name: self.name,
            status: self.status,
        }
    }
}