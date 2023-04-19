// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The volume mounts for a container for an Amazon EKS job. For more information about volumes and volume mounts in Kubernetes, see <a href="https://kubernetes.io/docs/concepts/storage/volumes/">Volumes</a> in the <i>Kubernetes documentation</i>.</p>
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
pub struct EksContainerVolumeMount {
    /// <p>The name the volume mount. This must match the name of one of the volumes in the pod.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The path on the container where the volume is mounted.</p>
    #[doc(hidden)]
    pub mount_path: std::option::Option<std::string::String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    #[doc(hidden)]
    pub read_only: std::option::Option<bool>,
}
impl EksContainerVolumeMount {
    /// <p>The name the volume mount. This must match the name of one of the volumes in the pod.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The path on the container where the volume is mounted.</p>
    pub fn mount_path(&self) -> std::option::Option<&str> {
        self.mount_path.as_deref()
    }
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    pub fn read_only(&self) -> std::option::Option<bool> {
        self.read_only
    }
}
impl EksContainerVolumeMount {
    /// Creates a new builder-style object to manufacture [`EksContainerVolumeMount`](crate::types::EksContainerVolumeMount).
    pub fn builder() -> crate::types::builders::EksContainerVolumeMountBuilder {
        crate::types::builders::EksContainerVolumeMountBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EksContainerVolumeMount;
/// A builder for [`EksContainerVolumeMount`](crate::types::EksContainerVolumeMount).
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
pub struct EksContainerVolumeMountBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) mount_path: std::option::Option<std::string::String>,
    pub(crate) read_only: std::option::Option<bool>,
}
impl EksContainerVolumeMountBuilder {
    /// <p>The name the volume mount. This must match the name of one of the volumes in the pod.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name the volume mount. This must match the name of one of the volumes in the pod.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The path on the container where the volume is mounted.</p>
    pub fn mount_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.mount_path = Some(input.into());
        self
    }
    /// <p>The path on the container where the volume is mounted.</p>
    pub fn set_mount_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.mount_path = input;
        self
    }
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    pub fn read_only(mut self, input: bool) -> Self {
        self.read_only = Some(input);
        self
    }
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    pub fn set_read_only(mut self, input: std::option::Option<bool>) -> Self {
        self.read_only = input;
        self
    }
    /// Consumes the builder and constructs a [`EksContainerVolumeMount`](crate::types::EksContainerVolumeMount).
    pub fn build(self) -> crate::types::EksContainerVolumeMount {
        crate::types::EksContainerVolumeMount {
            name: self.name,
            mount_path: self.mount_path,
            read_only: self.read_only,
        }
    }
}