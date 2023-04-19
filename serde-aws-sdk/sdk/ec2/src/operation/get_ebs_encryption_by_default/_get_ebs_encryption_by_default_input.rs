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
pub struct GetEbsEncryptionByDefaultInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl GetEbsEncryptionByDefaultInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl GetEbsEncryptionByDefaultInput {
    /// Creates a new builder-style object to manufacture [`GetEbsEncryptionByDefaultInput`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput).
    pub fn builder() -> crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultInputBuilder{
        crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput;
/// A builder for [`GetEbsEncryptionByDefaultInput`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput).
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
pub struct GetEbsEncryptionByDefaultInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
}
impl GetEbsEncryptionByDefaultInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`GetEbsEncryptionByDefaultInput`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultInput {
                dry_run: self.dry_run,
            },
        )
    }
}