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
pub struct EnableSerialConsoleAccessInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl EnableSerialConsoleAccessInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableSerialConsoleAccessInput {
    /// Creates a new builder-style object to manufacture [`EnableSerialConsoleAccessInput`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput).
    pub fn builder() -> crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessInputBuilder{
        crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput;
/// A builder for [`EnableSerialConsoleAccessInput`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput).
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
pub struct EnableSerialConsoleAccessInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
}
impl EnableSerialConsoleAccessInputBuilder {
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
    /// Consumes the builder and constructs a [`EnableSerialConsoleAccessInput`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::enable_serial_console_access::EnableSerialConsoleAccessInput {
                dry_run: self.dry_run,
            },
        )
    }
}