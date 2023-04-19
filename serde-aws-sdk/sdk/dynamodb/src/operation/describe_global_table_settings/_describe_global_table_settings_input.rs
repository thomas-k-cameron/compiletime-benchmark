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
pub struct DescribeGlobalTableSettingsInput {
    /// <p>The name of the global table to describe.</p>
    #[doc(hidden)]
    pub global_table_name: std::option::Option<std::string::String>,
}
impl DescribeGlobalTableSettingsInput {
    /// <p>The name of the global table to describe.</p>
    pub fn global_table_name(&self) -> std::option::Option<&str> {
        self.global_table_name.as_deref()
    }
}
impl DescribeGlobalTableSettingsInput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableSettingsInput`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput).
    pub fn builder() -> crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInputBuilder{
        crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput;
/// A builder for [`DescribeGlobalTableSettingsInput`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput).
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
pub struct DescribeGlobalTableSettingsInputBuilder {
    pub(crate) global_table_name: std::option::Option<std::string::String>,
}
impl DescribeGlobalTableSettingsInputBuilder {
    /// <p>The name of the global table to describe.</p>
    pub fn global_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.global_table_name = Some(input.into());
        self
    }
    /// <p>The name of the global table to describe.</p>
    pub fn set_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.global_table_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGlobalTableSettingsInput`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput {
                global_table_name: self.global_table_name,
            },
        )
    }
}
