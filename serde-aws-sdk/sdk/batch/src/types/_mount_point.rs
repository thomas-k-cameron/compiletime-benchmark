// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details for a Docker volume mount point that's used in a job's container properties. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.19/#create-a-container">Create a container</a> section of the <i>Docker Remote API</i> and the <code>--volume</code> option to docker run.</p>
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
pub struct MountPoint {
    /// <p>The path on the container where the host volume is mounted.</p>
    #[doc(hidden)]
    pub container_path: std::option::Option<std::string::String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    #[doc(hidden)]
    pub read_only: std::option::Option<bool>,
    /// <p>The name of the volume to mount.</p>
    #[doc(hidden)]
    pub source_volume: std::option::Option<std::string::String>,
}
impl MountPoint {
    /// <p>The path on the container where the host volume is mounted.</p>
    pub fn container_path(&self) -> std::option::Option<&str> {
        self.container_path.as_deref()
    }
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    pub fn read_only(&self) -> std::option::Option<bool> {
        self.read_only
    }
    /// <p>The name of the volume to mount.</p>
    pub fn source_volume(&self) -> std::option::Option<&str> {
        self.source_volume.as_deref()
    }
}
impl MountPoint {
    /// Creates a new builder-style object to manufacture [`MountPoint`](crate::types::MountPoint).
    pub fn builder() -> crate::types::builders::MountPointBuilder {
        crate::types::builders::MountPointBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::MountPoint;
/// A builder for [`MountPoint`](crate::types::MountPoint).
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
pub struct MountPointBuilder {
    pub(crate) container_path: std::option::Option<std::string::String>,
    pub(crate) read_only: std::option::Option<bool>,
    pub(crate) source_volume: std::option::Option<std::string::String>,
}
impl MountPointBuilder {
    /// <p>The path on the container where the host volume is mounted.</p>
    pub fn container_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.container_path = Some(input.into());
        self
    }
    /// <p>The path on the container where the host volume is mounted.</p>
    pub fn set_container_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.container_path = input;
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
    /// <p>The name of the volume to mount.</p>
    pub fn source_volume(mut self, input: impl Into<std::string::String>) -> Self {
        self.source_volume = Some(input.into());
        self
    }
    /// <p>The name of the volume to mount.</p>
    pub fn set_source_volume(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.source_volume = input;
        self
    }
    /// Consumes the builder and constructs a [`MountPoint`](crate::types::MountPoint).
    pub fn build(self) -> crate::types::MountPoint {
        crate::types::MountPoint {
            container_path: self.container_path,
            read_only: self.read_only,
            source_volume: self.source_volume,
        }
    }
}
