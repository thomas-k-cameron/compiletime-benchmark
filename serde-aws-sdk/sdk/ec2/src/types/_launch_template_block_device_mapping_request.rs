// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a block device mapping.</p>
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
pub struct LaunchTemplateBlockDeviceMappingRequest {
    /// <p>The device name (for example, /dev/sdh or xvdh).</p>
    #[doc(hidden)]
    pub device_name: std::option::Option<std::string::String>,
    /// <p>The virtual device name (ephemeralN). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for ephemeral0 and ephemeral1. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    #[doc(hidden)]
    pub virtual_name: std::option::Option<std::string::String>,
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    #[doc(hidden)]
    pub ebs: std::option::Option<crate::types::LaunchTemplateEbsBlockDeviceRequest>,
    /// <p>To omit the device from the block device mapping, specify an empty string.</p>
    #[doc(hidden)]
    pub no_device: std::option::Option<std::string::String>,
}
impl LaunchTemplateBlockDeviceMappingRequest {
    /// <p>The device name (for example, /dev/sdh or xvdh).</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The virtual device name (ephemeralN). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for ephemeral0 and ephemeral1. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    pub fn virtual_name(&self) -> std::option::Option<&str> {
        self.virtual_name.as_deref()
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(&self) -> std::option::Option<&crate::types::LaunchTemplateEbsBlockDeviceRequest> {
        self.ebs.as_ref()
    }
    /// <p>To omit the device from the block device mapping, specify an empty string.</p>
    pub fn no_device(&self) -> std::option::Option<&str> {
        self.no_device.as_deref()
    }
}
impl LaunchTemplateBlockDeviceMappingRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateBlockDeviceMappingRequest`](crate::types::LaunchTemplateBlockDeviceMappingRequest).
    pub fn builder() -> crate::types::builders::LaunchTemplateBlockDeviceMappingRequestBuilder {
        crate::types::builders::LaunchTemplateBlockDeviceMappingRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateBlockDeviceMappingRequest;
/// A builder for [`LaunchTemplateBlockDeviceMappingRequest`](crate::types::LaunchTemplateBlockDeviceMappingRequest).
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
pub struct LaunchTemplateBlockDeviceMappingRequestBuilder {
    pub(crate) device_name: std::option::Option<std::string::String>,
    pub(crate) virtual_name: std::option::Option<std::string::String>,
    pub(crate) ebs: std::option::Option<crate::types::LaunchTemplateEbsBlockDeviceRequest>,
    pub(crate) no_device: std::option::Option<std::string::String>,
}
impl LaunchTemplateBlockDeviceMappingRequestBuilder {
    /// <p>The device name (for example, /dev/sdh or xvdh).</p>
    pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.device_name = Some(input.into());
        self
    }
    /// <p>The device name (for example, /dev/sdh or xvdh).</p>
    pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.device_name = input;
        self
    }
    /// <p>The virtual device name (ephemeralN). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for ephemeral0 and ephemeral1. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    pub fn virtual_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.virtual_name = Some(input.into());
        self
    }
    /// <p>The virtual device name (ephemeralN). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for ephemeral0 and ephemeral1. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    pub fn set_virtual_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.virtual_name = input;
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(mut self, input: crate::types::LaunchTemplateEbsBlockDeviceRequest) -> Self {
        self.ebs = Some(input);
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn set_ebs(
        mut self,
        input: std::option::Option<crate::types::LaunchTemplateEbsBlockDeviceRequest>,
    ) -> Self {
        self.ebs = input;
        self
    }
    /// <p>To omit the device from the block device mapping, specify an empty string.</p>
    pub fn no_device(mut self, input: impl Into<std::string::String>) -> Self {
        self.no_device = Some(input.into());
        self
    }
    /// <p>To omit the device from the block device mapping, specify an empty string.</p>
    pub fn set_no_device(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.no_device = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateBlockDeviceMappingRequest`](crate::types::LaunchTemplateBlockDeviceMappingRequest).
    pub fn build(self) -> crate::types::LaunchTemplateBlockDeviceMappingRequest {
        crate::types::LaunchTemplateBlockDeviceMappingRequest {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
            ebs: self.ebs,
            no_device: self.no_device,
        }
    }
}
