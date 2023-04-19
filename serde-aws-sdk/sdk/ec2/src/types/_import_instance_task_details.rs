// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an import instance task.</p>
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
pub struct ImportInstanceTaskDetails {
    /// <p>A description of the task.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The instance operating system.</p>
    #[doc(hidden)]
    pub platform: std::option::Option<crate::types::PlatformValues>,
    /// <p>The volumes.</p>
    #[doc(hidden)]
    pub volumes: std::option::Option<std::vec::Vec<crate::types::ImportInstanceVolumeDetailItem>>,
}
impl ImportInstanceTaskDetails {
    /// <p>A description of the task.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The instance operating system.</p>
    pub fn platform(&self) -> std::option::Option<&crate::types::PlatformValues> {
        self.platform.as_ref()
    }
    /// <p>The volumes.</p>
    pub fn volumes(&self) -> std::option::Option<&[crate::types::ImportInstanceVolumeDetailItem]> {
        self.volumes.as_deref()
    }
}
impl ImportInstanceTaskDetails {
    /// Creates a new builder-style object to manufacture [`ImportInstanceTaskDetails`](crate::types::ImportInstanceTaskDetails).
    pub fn builder() -> crate::types::builders::ImportInstanceTaskDetailsBuilder {
        crate::types::builders::ImportInstanceTaskDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ImportInstanceTaskDetails;
/// A builder for [`ImportInstanceTaskDetails`](crate::types::ImportInstanceTaskDetails).
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
pub struct ImportInstanceTaskDetailsBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) platform: std::option::Option<crate::types::PlatformValues>,
    pub(crate) volumes:
        std::option::Option<std::vec::Vec<crate::types::ImportInstanceVolumeDetailItem>>,
}
impl ImportInstanceTaskDetailsBuilder {
    /// <p>A description of the task.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the task.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The instance operating system.</p>
    pub fn platform(mut self, input: crate::types::PlatformValues) -> Self {
        self.platform = Some(input);
        self
    }
    /// <p>The instance operating system.</p>
    pub fn set_platform(
        mut self,
        input: std::option::Option<crate::types::PlatformValues>,
    ) -> Self {
        self.platform = input;
        self
    }
    /// Appends an item to `volumes`.
    ///
    /// To override the contents of this collection use [`set_volumes`](Self::set_volumes).
    ///
    /// <p>The volumes.</p>
    pub fn volumes(mut self, input: crate::types::ImportInstanceVolumeDetailItem) -> Self {
        let mut v = self.volumes.unwrap_or_default();
        v.push(input);
        self.volumes = Some(v);
        self
    }
    /// <p>The volumes.</p>
    pub fn set_volumes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ImportInstanceVolumeDetailItem>>,
    ) -> Self {
        self.volumes = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportInstanceTaskDetails`](crate::types::ImportInstanceTaskDetails).
    pub fn build(self) -> crate::types::ImportInstanceTaskDetails {
        crate::types::ImportInstanceTaskDetails {
            description: self.description,
            instance_id: self.instance_id,
            platform: self.platform,
            volumes: self.volumes,
        }
    }
}
