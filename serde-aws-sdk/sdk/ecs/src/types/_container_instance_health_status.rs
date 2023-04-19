// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object representing the health status of the container instance.</p>
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
pub struct ContainerInstanceHealthStatus {
    /// <p>The overall health status of the container instance. This is an aggregate status of all container instance health checks.</p>
    #[doc(hidden)]
    pub overall_status: std::option::Option<crate::types::InstanceHealthCheckState>,
    /// <p>An array of objects representing the details of the container instance health status.</p>
    #[doc(hidden)]
    pub details: std::option::Option<std::vec::Vec<crate::types::InstanceHealthCheckResult>>,
}
impl ContainerInstanceHealthStatus {
    /// <p>The overall health status of the container instance. This is an aggregate status of all container instance health checks.</p>
    pub fn overall_status(&self) -> std::option::Option<&crate::types::InstanceHealthCheckState> {
        self.overall_status.as_ref()
    }
    /// <p>An array of objects representing the details of the container instance health status.</p>
    pub fn details(&self) -> std::option::Option<&[crate::types::InstanceHealthCheckResult]> {
        self.details.as_deref()
    }
}
impl ContainerInstanceHealthStatus {
    /// Creates a new builder-style object to manufacture [`ContainerInstanceHealthStatus`](crate::types::ContainerInstanceHealthStatus).
    pub fn builder() -> crate::types::builders::ContainerInstanceHealthStatusBuilder {
        crate::types::builders::ContainerInstanceHealthStatusBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ContainerInstanceHealthStatus;
/// A builder for [`ContainerInstanceHealthStatus`](crate::types::ContainerInstanceHealthStatus).
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
pub struct ContainerInstanceHealthStatusBuilder {
    pub(crate) overall_status: std::option::Option<crate::types::InstanceHealthCheckState>,
    pub(crate) details: std::option::Option<std::vec::Vec<crate::types::InstanceHealthCheckResult>>,
}
impl ContainerInstanceHealthStatusBuilder {
    /// <p>The overall health status of the container instance. This is an aggregate status of all container instance health checks.</p>
    pub fn overall_status(mut self, input: crate::types::InstanceHealthCheckState) -> Self {
        self.overall_status = Some(input);
        self
    }
    /// <p>The overall health status of the container instance. This is an aggregate status of all container instance health checks.</p>
    pub fn set_overall_status(
        mut self,
        input: std::option::Option<crate::types::InstanceHealthCheckState>,
    ) -> Self {
        self.overall_status = input;
        self
    }
    /// Appends an item to `details`.
    ///
    /// To override the contents of this collection use [`set_details`](Self::set_details).
    ///
    /// <p>An array of objects representing the details of the container instance health status.</p>
    pub fn details(mut self, input: crate::types::InstanceHealthCheckResult) -> Self {
        let mut v = self.details.unwrap_or_default();
        v.push(input);
        self.details = Some(v);
        self
    }
    /// <p>An array of objects representing the details of the container instance health status.</p>
    pub fn set_details(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceHealthCheckResult>>,
    ) -> Self {
        self.details = input;
        self
    }
    /// Consumes the builder and constructs a [`ContainerInstanceHealthStatus`](crate::types::ContainerInstanceHealthStatus).
    pub fn build(self) -> crate::types::ContainerInstanceHealthStatus {
        crate::types::ContainerInstanceHealthStatus {
            overall_status: self.overall_status,
            details: self.details,
        }
    }
}
