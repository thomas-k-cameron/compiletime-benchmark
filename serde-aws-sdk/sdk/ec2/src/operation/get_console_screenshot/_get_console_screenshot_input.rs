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
pub struct GetConsoleScreenshotInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>When set to <code>true</code>, acts as keystroke input and wakes up an instance that's in standby or "sleep" mode.</p>
    #[doc(hidden)]
    pub wake_up: std::option::Option<bool>,
}
impl GetConsoleScreenshotInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>When set to <code>true</code>, acts as keystroke input and wakes up an instance that's in standby or "sleep" mode.</p>
    pub fn wake_up(&self) -> std::option::Option<bool> {
        self.wake_up
    }
}
impl GetConsoleScreenshotInput {
    /// Creates a new builder-style object to manufacture [`GetConsoleScreenshotInput`](crate::operation::get_console_screenshot::GetConsoleScreenshotInput).
    pub fn builder(
    ) -> crate::operation::get_console_screenshot::builders::GetConsoleScreenshotInputBuilder {
        crate::operation::get_console_screenshot::builders::GetConsoleScreenshotInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_console_screenshot::GetConsoleScreenshotInput;
/// A builder for [`GetConsoleScreenshotInput`](crate::operation::get_console_screenshot::GetConsoleScreenshotInput).
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
pub struct GetConsoleScreenshotInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) wake_up: std::option::Option<bool>,
}
impl GetConsoleScreenshotInputBuilder {
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
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>When set to <code>true</code>, acts as keystroke input and wakes up an instance that's in standby or "sleep" mode.</p>
    pub fn wake_up(mut self, input: bool) -> Self {
        self.wake_up = Some(input);
        self
    }
    /// <p>When set to <code>true</code>, acts as keystroke input and wakes up an instance that's in standby or "sleep" mode.</p>
    pub fn set_wake_up(mut self, input: std::option::Option<bool>) -> Self {
        self.wake_up = input;
        self
    }
    /// Consumes the builder and constructs a [`GetConsoleScreenshotInput`](crate::operation::get_console_screenshot::GetConsoleScreenshotInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_console_screenshot::GetConsoleScreenshotInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_console_screenshot::GetConsoleScreenshotInput {
                dry_run: self.dry_run,
                instance_id: self.instance_id,
                wake_up: self.wake_up,
            },
        )
    }
}
