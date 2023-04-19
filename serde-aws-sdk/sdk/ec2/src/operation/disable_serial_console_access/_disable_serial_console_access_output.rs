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
pub struct DisableSerialConsoleAccessOutput {
    /// <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    #[doc(hidden)]
    pub serial_console_access_enabled: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableSerialConsoleAccessOutput {
    /// <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    pub fn serial_console_access_enabled(&self) -> std::option::Option<bool> {
        self.serial_console_access_enabled
    }
}
impl aws_http::request_id::RequestId for DisableSerialConsoleAccessOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableSerialConsoleAccessOutput {
    /// Creates a new builder-style object to manufacture [`DisableSerialConsoleAccessOutput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput).
    pub fn builder() -> crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessOutputBuilder{
        crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput;
/// A builder for [`DisableSerialConsoleAccessOutput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput).
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
pub struct DisableSerialConsoleAccessOutputBuilder {
    pub(crate) serial_console_access_enabled: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableSerialConsoleAccessOutputBuilder {
    /// <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    pub fn serial_console_access_enabled(mut self, input: bool) -> Self {
        self.serial_console_access_enabled = Some(input);
        self
    }
    /// <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    pub fn set_serial_console_access_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.serial_console_access_enabled = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableSerialConsoleAccessOutput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput).
    pub fn build(
        self,
    ) -> crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput {
        crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput {
            serial_console_access_enabled: self.serial_console_access_enabled,
            _request_id: self._request_id,
        }
    }
}
