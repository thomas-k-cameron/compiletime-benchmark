// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains details about how a service-linked role is used, if that information is returned by the service.</p>
/// <p>This data type is used as a response element in the <code>GetServiceLinkedRoleDeletionStatus</code> operation.</p>
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
pub struct RoleUsageType {
    /// <p>The name of the Region where the service-linked role is being used.</p>
    #[doc(hidden)]
    pub region: std::option::Option<std::string::String>,
    /// <p>The name of the resource that is using the service-linked role.</p>
    #[doc(hidden)]
    pub resources: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl RoleUsageType {
    /// <p>The name of the Region where the service-linked role is being used.</p>
    pub fn region(&self) -> std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p>The name of the resource that is using the service-linked role.</p>
    pub fn resources(&self) -> std::option::Option<&[std::string::String]> {
        self.resources.as_deref()
    }
}
impl RoleUsageType {
    /// Creates a new builder-style object to manufacture [`RoleUsageType`](crate::types::RoleUsageType).
    pub fn builder() -> crate::types::builders::RoleUsageTypeBuilder {
        crate::types::builders::RoleUsageTypeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RoleUsageType;
/// A builder for [`RoleUsageType`](crate::types::RoleUsageType).
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
pub struct RoleUsageTypeBuilder {
    pub(crate) region: std::option::Option<std::string::String>,
    pub(crate) resources: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl RoleUsageTypeBuilder {
    /// <p>The name of the Region where the service-linked role is being used.</p>
    pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
        self.region = Some(input.into());
        self
    }
    /// <p>The name of the Region where the service-linked role is being used.</p>
    pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// Appends an item to `resources`.
    ///
    /// To override the contents of this collection use [`set_resources`](Self::set_resources).
    ///
    /// <p>The name of the resource that is using the service-linked role.</p>
    pub fn resources(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resources.unwrap_or_default();
        v.push(input.into());
        self.resources = Some(v);
        self
    }
    /// <p>The name of the resource that is using the service-linked role.</p>
    pub fn set_resources(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resources = input;
        self
    }
    /// Consumes the builder and constructs a [`RoleUsageType`](crate::types::RoleUsageType).
    pub fn build(self) -> crate::types::RoleUsageType {
        crate::types::RoleUsageType {
            region: self.region,
            resources: self.resources,
        }
    }
}
