// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a block device mapping entry.</p>
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
pub struct InstanceBlockDeviceMappingSpecification {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    #[doc(hidden)]
    pub device_name: std::option::Option<std::string::String>,
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    #[doc(hidden)]
    pub ebs: std::option::Option<crate::types::EbsInstanceBlockDeviceSpecification>,
    /// <p>suppress the specified device included in the block device mapping.</p>
    #[doc(hidden)]
    pub no_device: std::option::Option<std::string::String>,
    /// <p>The virtual device name.</p>
    #[doc(hidden)]
    pub virtual_name: std::option::Option<std::string::String>,
}
impl InstanceBlockDeviceMappingSpecification {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(&self) -> std::option::Option<&crate::types::EbsInstanceBlockDeviceSpecification> {
        self.ebs.as_ref()
    }
    /// <p>suppress the specified device included in the block device mapping.</p>
    pub fn no_device(&self) -> std::option::Option<&str> {
        self.no_device.as_deref()
    }
    /// <p>The virtual device name.</p>
    pub fn virtual_name(&self) -> std::option::Option<&str> {
        self.virtual_name.as_deref()
    }
}
impl InstanceBlockDeviceMappingSpecification {
    /// Creates a new builder-style object to manufacture [`InstanceBlockDeviceMappingSpecification`](crate::types::InstanceBlockDeviceMappingSpecification).
    pub fn builder() -> crate::types::builders::InstanceBlockDeviceMappingSpecificationBuilder {
        crate::types::builders::InstanceBlockDeviceMappingSpecificationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceBlockDeviceMappingSpecification;
/// A builder for [`InstanceBlockDeviceMappingSpecification`](crate::types::InstanceBlockDeviceMappingSpecification).
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
pub struct InstanceBlockDeviceMappingSpecificationBuilder {
    pub(crate) device_name: std::option::Option<std::string::String>,
    pub(crate) ebs: std::option::Option<crate::types::EbsInstanceBlockDeviceSpecification>,
    pub(crate) no_device: std::option::Option<std::string::String>,
    pub(crate) virtual_name: std::option::Option<std::string::String>,
}
impl InstanceBlockDeviceMappingSpecificationBuilder {
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
    pub fn ebs(mut self, input: crate::types::EbsInstanceBlockDeviceSpecification) -> Self {
        self.ebs = Some(input);
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn set_ebs(
        mut self,
        input: std::option::Option<crate::types::EbsInstanceBlockDeviceSpecification>,
    ) -> Self {
        self.ebs = input;
        self
    }
    /// <p>suppress the specified device included in the block device mapping.</p>
    pub fn no_device(mut self, input: impl Into<std::string::String>) -> Self {
        self.no_device = Some(input.into());
        self
    }
    /// <p>suppress the specified device included in the block device mapping.</p>
    pub fn set_no_device(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.no_device = input;
        self
    }
    /// <p>The virtual device name.</p>
    pub fn virtual_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.virtual_name = Some(input.into());
        self
    }
    /// <p>The virtual device name.</p>
    pub fn set_virtual_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.virtual_name = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceBlockDeviceMappingSpecification`](crate::types::InstanceBlockDeviceMappingSpecification).
    pub fn build(self) -> crate::types::InstanceBlockDeviceMappingSpecification {
        crate::types::InstanceBlockDeviceMappingSpecification {
            device_name: self.device_name,
            ebs: self.ebs,
            no_device: self.no_device,
            virtual_name: self.virtual_name,
        }
    }
}
