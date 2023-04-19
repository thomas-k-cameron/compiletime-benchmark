// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>If a health check or hosted zone was created by another service, <code>LinkedService</code> is a complex type that describes the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
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
pub struct LinkedService {
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    #[doc(hidden)]
    pub service_principal: std::option::Option<std::string::String>,
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
}
impl LinkedService {
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn service_principal(&self) -> std::option::Option<&str> {
        self.service_principal.as_deref()
    }
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl LinkedService {
    /// Creates a new builder-style object to manufacture [`LinkedService`](crate::types::LinkedService).
    pub fn builder() -> crate::types::builders::LinkedServiceBuilder {
        crate::types::builders::LinkedServiceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LinkedService;
/// A builder for [`LinkedService`](crate::types::LinkedService).
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
pub struct LinkedServiceBuilder {
    pub(crate) service_principal: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
}
impl LinkedServiceBuilder {
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn service_principal(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_principal = Some(input.into());
        self
    }
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn set_service_principal(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.service_principal = input;
        self
    }
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`LinkedService`](crate::types::LinkedService).
    pub fn build(self) -> crate::types::LinkedService {
        crate::types::LinkedService {
            service_principal: self.service_principal,
            description: self.description,
        }
    }
}
