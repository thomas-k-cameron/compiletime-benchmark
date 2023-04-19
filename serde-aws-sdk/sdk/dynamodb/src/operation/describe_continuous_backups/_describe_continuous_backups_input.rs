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
pub struct DescribeContinuousBackupsInput {
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    #[doc(hidden)]
    pub table_name: std::option::Option<std::string::String>,
}
impl DescribeContinuousBackupsInput {
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    pub fn table_name(&self) -> std::option::Option<&str> {
        self.table_name.as_deref()
    }
}
impl DescribeContinuousBackupsInput {
    /// Creates a new builder-style object to manufacture [`DescribeContinuousBackupsInput`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput).
    pub fn builder() -> crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder{
        crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput;
/// A builder for [`DescribeContinuousBackupsInput`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput).
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
pub struct DescribeContinuousBackupsInputBuilder {
    pub(crate) table_name: std::option::Option<std::string::String>,
}
impl DescribeContinuousBackupsInputBuilder {
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.table_name = Some(input.into());
        self
    }
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeContinuousBackupsInput`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput {
                table_name: self.table_name,
            },
        )
    }
}
