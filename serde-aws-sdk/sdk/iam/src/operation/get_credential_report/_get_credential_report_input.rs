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
pub struct GetCredentialReportInput {}
impl GetCredentialReportInput {
    /// Creates a new builder-style object to manufacture [`GetCredentialReportInput`](crate::operation::get_credential_report::GetCredentialReportInput).
    pub fn builder(
    ) -> crate::operation::get_credential_report::builders::GetCredentialReportInputBuilder {
        crate::operation::get_credential_report::builders::GetCredentialReportInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_credential_report::GetCredentialReportInput;
/// A builder for [`GetCredentialReportInput`](crate::operation::get_credential_report::GetCredentialReportInput).
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
pub struct GetCredentialReportInputBuilder {}
impl GetCredentialReportInputBuilder {
    /// Consumes the builder and constructs a [`GetCredentialReportInput`](crate::operation::get_credential_report::GetCredentialReportInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_credential_report::GetCredentialReportInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_credential_report::GetCredentialReportInput {})
    }
}