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
pub struct InstanceBlockDeviceMapping {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    #[doc(hidden)]
    pub device_name: std::option::Option<std::string::String>,
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    #[doc(hidden)]
    pub ebs: std::option::Option<crate::types::EbsInstanceBlockDevice>,
}
impl InstanceBlockDeviceMapping {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(&self) -> std::option::Option<&crate::types::EbsInstanceBlockDevice> {
        self.ebs.as_ref()
    }
}
impl InstanceBlockDeviceMapping {
    /// Creates a new builder-style object to manufacture [`InstanceBlockDeviceMapping`](crate::types::InstanceBlockDeviceMapping).
    pub fn builder() -> crate::types::builders::InstanceBlockDeviceMappingBuilder {
        crate::types::builders::InstanceBlockDeviceMappingBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceBlockDeviceMapping;
/// A builder for [`InstanceBlockDeviceMapping`](crate::types::InstanceBlockDeviceMapping).
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
pub struct InstanceBlockDeviceMappingBuilder {
    pub(crate) device_name: std::option::Option<std::string::String>,
    pub(crate) ebs: std::option::Option<crate::types::EbsInstanceBlockDevice>,
}
impl InstanceBlockDeviceMappingBuilder {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.device_name = Some(input.into());
        self
    }
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.device_name = input;
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(mut self, input: crate::types::EbsInstanceBlockDevice) -> Self {
        self.ebs = Some(input);
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn set_ebs(
        mut self,
        input: std::option::Option<crate::types::EbsInstanceBlockDevice>,
    ) -> Self {
        self.ebs = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceBlockDeviceMapping`](crate::types::InstanceBlockDeviceMapping).
    pub fn build(self) -> crate::types::InstanceBlockDeviceMapping {
        crate::types::InstanceBlockDeviceMapping {
            device_name: self.device_name,
            ebs: self.ebs,
        }
    }
}
