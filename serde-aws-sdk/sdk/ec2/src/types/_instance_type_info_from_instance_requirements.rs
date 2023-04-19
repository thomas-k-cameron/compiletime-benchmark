// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The list of instance types with the specified instance attributes.</p>
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
pub struct InstanceTypeInfoFromInstanceRequirements {
    /// <p>The matching instance type.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
}
impl InstanceTypeInfoFromInstanceRequirements {
    /// <p>The matching instance type.</p>
    pub fn instance_type(&self) -> std::option::Option<&str> {
        self.instance_type.as_deref()
    }
}
impl InstanceTypeInfoFromInstanceRequirements {
    /// Creates a new builder-style object to manufacture [`InstanceTypeInfoFromInstanceRequirements`](crate::types::InstanceTypeInfoFromInstanceRequirements).
    pub fn builder() -> crate::types::builders::InstanceTypeInfoFromInstanceRequirementsBuilder {
        crate::types::builders::InstanceTypeInfoFromInstanceRequirementsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceTypeInfoFromInstanceRequirements;
/// A builder for [`InstanceTypeInfoFromInstanceRequirements`](crate::types::InstanceTypeInfoFromInstanceRequirements).
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
pub struct InstanceTypeInfoFromInstanceRequirementsBuilder {
    pub(crate) instance_type: std::option::Option<std::string::String>,
}
impl InstanceTypeInfoFromInstanceRequirementsBuilder {
    /// <p>The matching instance type.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_type = Some(input.into());
        self
    }
    /// <p>The matching instance type.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceTypeInfoFromInstanceRequirements`](crate::types::InstanceTypeInfoFromInstanceRequirements).
    pub fn build(self) -> crate::types::InstanceTypeInfoFromInstanceRequirements {
        crate::types::InstanceTypeInfoFromInstanceRequirements {
            instance_type: self.instance_type,
        }
    }
}
