// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object with the name of the retention configuration and the retention period in days. The object stores the configuration for data retention in Config.</p>
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
pub struct RetentionConfiguration {
    /// <p>The name of the retention configuration object.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    #[doc(hidden)]
    pub retention_period_in_days: i32,
}
impl RetentionConfiguration {
    /// <p>The name of the retention configuration object.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn retention_period_in_days(&self) -> i32 {
        self.retention_period_in_days
    }
}
impl RetentionConfiguration {
    /// Creates a new builder-style object to manufacture [`RetentionConfiguration`](crate::types::RetentionConfiguration).
    pub fn builder() -> crate::types::builders::RetentionConfigurationBuilder {
        crate::types::builders::RetentionConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RetentionConfiguration;
/// A builder for [`RetentionConfiguration`](crate::types::RetentionConfiguration).
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
pub struct RetentionConfigurationBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) retention_period_in_days: std::option::Option<i32>,
}
impl RetentionConfigurationBuilder {
    /// <p>The name of the retention configuration object.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the retention configuration object.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn retention_period_in_days(mut self, input: i32) -> Self {
        self.retention_period_in_days = Some(input);
        self
    }
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn set_retention_period_in_days(mut self, input: std::option::Option<i32>) -> Self {
        self.retention_period_in_days = input;
        self
    }
    /// Consumes the builder and constructs a [`RetentionConfiguration`](crate::types::RetentionConfiguration).
    pub fn build(self) -> crate::types::RetentionConfiguration {
        crate::types::RetentionConfiguration {
            name: self.name,
            retention_period_in_days: self.retention_period_in_days.unwrap_or_default(),
        }
    }
}
