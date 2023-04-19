// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct PutRetentionConfigurationInput {
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    #[doc(hidden)]
    pub retention_period_in_days: i32,
}
impl PutRetentionConfigurationInput {
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn retention_period_in_days(&self) -> i32 {
        self.retention_period_in_days
    }
}
impl PutRetentionConfigurationInput {
    /// Creates a new builder-style object to manufacture [`PutRetentionConfigurationInput`](crate::operation::put_retention_configuration::PutRetentionConfigurationInput).
    pub fn builder() -> crate::operation::put_retention_configuration::builders::PutRetentionConfigurationInputBuilder{
        crate::operation::put_retention_configuration::builders::PutRetentionConfigurationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_retention_configuration::PutRetentionConfigurationInput;
/// A builder for [`PutRetentionConfigurationInput`](crate::operation::put_retention_configuration::PutRetentionConfigurationInput).
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
pub struct PutRetentionConfigurationInputBuilder {
    pub(crate) retention_period_in_days: std::option::Option<i32>,
}
impl PutRetentionConfigurationInputBuilder {
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
    /// Consumes the builder and constructs a [`PutRetentionConfigurationInput`](crate::operation::put_retention_configuration::PutRetentionConfigurationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_retention_configuration::PutRetentionConfigurationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_retention_configuration::PutRetentionConfigurationInput {
                retention_period_in_days: self.retention_period_in_days.unwrap_or_default(),
            },
        )
    }
}