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
pub struct WithdrawByoipCidrInput {
    /// <p>The address range, in CIDR notation.</p>
    #[doc(hidden)]
    pub cidr: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl WithdrawByoipCidrInput {
    /// <p>The address range, in CIDR notation.</p>
    pub fn cidr(&self) -> std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl WithdrawByoipCidrInput {
    /// Creates a new builder-style object to manufacture [`WithdrawByoipCidrInput`](crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput).
    pub fn builder(
    ) -> crate::operation::withdraw_byoip_cidr::builders::WithdrawByoipCidrInputBuilder {
        crate::operation::withdraw_byoip_cidr::builders::WithdrawByoipCidrInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput;
/// A builder for [`WithdrawByoipCidrInput`](crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput).
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
pub struct WithdrawByoipCidrInputBuilder {
    pub(crate) cidr: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl WithdrawByoipCidrInputBuilder {
    /// <p>The address range, in CIDR notation.</p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr = Some(input.into());
        self
    }
    /// <p>The address range, in CIDR notation.</p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr = input;
        self
    }
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
    /// Consumes the builder and constructs a [`WithdrawByoipCidrInput`](crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::withdraw_byoip_cidr::WithdrawByoipCidrInput {
                cidr: self.cidr,
                dry_run: self.dry_run,
            },
        )
    }
}
