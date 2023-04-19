// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object representing a container instance host device.</p>
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
pub struct Device {
    /// <p>The path for the device on the host container instance.</p>
    #[doc(hidden)]
    pub host_path: std::option::Option<std::string::String>,
    /// <p>The path inside the container at which to expose the host device.</p>
    #[doc(hidden)]
    pub container_path: std::option::Option<std::string::String>,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    #[doc(hidden)]
    pub permissions: std::option::Option<std::vec::Vec<crate::types::DeviceCgroupPermission>>,
}
impl Device {
    /// <p>The path for the device on the host container instance.</p>
    pub fn host_path(&self) -> std::option::Option<&str> {
        self.host_path.as_deref()
    }
    /// <p>The path inside the container at which to expose the host device.</p>
    pub fn container_path(&self) -> std::option::Option<&str> {
        self.container_path.as_deref()
    }
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    pub fn permissions(&self) -> std::option::Option<&[crate::types::DeviceCgroupPermission]> {
        self.permissions.as_deref()
    }
}
impl Device {
    /// Creates a new builder-style object to manufacture [`Device`](crate::types::Device).
    pub fn builder() -> crate::types::builders::DeviceBuilder {
        crate::types::builders::DeviceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Device;
/// A builder for [`Device`](crate::types::Device).
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
pub struct DeviceBuilder {
    pub(crate) host_path: std::option::Option<std::string::String>,
    pub(crate) container_path: std::option::Option<std::string::String>,
    pub(crate) permissions:
        std::option::Option<std::vec::Vec<crate::types::DeviceCgroupPermission>>,
}
impl DeviceBuilder {
    /// <p>The path for the device on the host container instance.</p>
    pub fn host_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.host_path = Some(input.into());
        self
    }
    /// <p>The path for the device on the host container instance.</p>
    pub fn set_host_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.host_path = input;
        self
    }
    /// <p>The path inside the container at which to expose the host device.</p>
    pub fn container_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.container_path = Some(input.into());
        self
    }
    /// <p>The path inside the container at which to expose the host device.</p>
    pub fn set_container_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.container_path = input;
        self
    }
    /// Appends an item to `permissions`.
    ///
    /// To override the contents of this collection use [`set_permissions`](Self::set_permissions).
    ///
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    pub fn permissions(mut self, input: crate::types::DeviceCgroupPermission) -> Self {
        let mut v = self.permissions.unwrap_or_default();
        v.push(input);
        self.permissions = Some(v);
        self
    }
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    pub fn set_permissions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::DeviceCgroupPermission>>,
    ) -> Self {
        self.permissions = input;
        self
    }
    /// Consumes the builder and constructs a [`Device`](crate::types::Device).
    pub fn build(self) -> crate::types::Device {
        crate::types::Device {
            host_path: self.host_path,
            container_path: self.container_path,
            permissions: self.permissions,
        }
    }
}
