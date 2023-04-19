// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the Convertible Reserved Instance offering.</p>
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
pub struct TargetConfiguration {
    /// <p>The number of instances the Convertible Reserved Instance offering can be applied to. This parameter is reserved and cannot be specified in a request</p>
    #[doc(hidden)]
    pub instance_count: std::option::Option<i32>,
    /// <p>The ID of the Convertible Reserved Instance offering.</p>
    #[doc(hidden)]
    pub offering_id: std::option::Option<std::string::String>,
}
impl TargetConfiguration {
    /// <p>The number of instances the Convertible Reserved Instance offering can be applied to. This parameter is reserved and cannot be specified in a request</p>
    pub fn instance_count(&self) -> std::option::Option<i32> {
        self.instance_count
    }
    /// <p>The ID of the Convertible Reserved Instance offering.</p>
    pub fn offering_id(&self) -> std::option::Option<&str> {
        self.offering_id.as_deref()
    }
}
impl TargetConfiguration {
    /// Creates a new builder-style object to manufacture [`TargetConfiguration`](crate::types::TargetConfiguration).
    pub fn builder() -> crate::types::builders::TargetConfigurationBuilder {
        crate::types::builders::TargetConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TargetConfiguration;
/// A builder for [`TargetConfiguration`](crate::types::TargetConfiguration).
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
pub struct TargetConfigurationBuilder {
    pub(crate) instance_count: std::option::Option<i32>,
    pub(crate) offering_id: std::option::Option<std::string::String>,
}
impl TargetConfigurationBuilder {
    /// <p>The number of instances the Convertible Reserved Instance offering can be applied to. This parameter is reserved and cannot be specified in a request</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = Some(input);
        self
    }
    /// <p>The number of instances the Convertible Reserved Instance offering can be applied to. This parameter is reserved and cannot be specified in a request</p>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// <p>The ID of the Convertible Reserved Instance offering.</p>
    pub fn offering_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.offering_id = Some(input.into());
        self
    }
    /// <p>The ID of the Convertible Reserved Instance offering.</p>
    pub fn set_offering_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.offering_id = input;
        self
    }
    /// Consumes the builder and constructs a [`TargetConfiguration`](crate::types::TargetConfiguration).
    pub fn build(self) -> crate::types::TargetConfiguration {
        crate::types::TargetConfiguration {
            instance_count: self.instance_count,
            offering_id: self.offering_id,
        }
    }
}
