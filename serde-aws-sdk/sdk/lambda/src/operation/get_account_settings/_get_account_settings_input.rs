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
pub struct GetAccountSettingsInput {}
impl GetAccountSettingsInput {
    /// Creates a new builder-style object to manufacture [`GetAccountSettingsInput`](crate::operation::get_account_settings::GetAccountSettingsInput).
    pub fn builder(
    ) -> crate::operation::get_account_settings::builders::GetAccountSettingsInputBuilder {
        crate::operation::get_account_settings::builders::GetAccountSettingsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_account_settings::GetAccountSettingsInput;
/// A builder for [`GetAccountSettingsInput`](crate::operation::get_account_settings::GetAccountSettingsInput).
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
pub struct GetAccountSettingsInputBuilder {}
impl GetAccountSettingsInputBuilder {
    /// Consumes the builder and constructs a [`GetAccountSettingsInput`](crate::operation::get_account_settings::GetAccountSettingsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_account_settings::GetAccountSettingsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_account_settings::GetAccountSettingsInput {})
    }
}