// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableSerialConsoleAccess`](crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DisableSerialConsoleAccessOutput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput) with field(s):
    ///   - [`serial_console_access_enabled(Option<bool>)`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessOutput::serial_console_access_enabled): <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    /// - On failure, responds with [`SdkError<DisableSerialConsoleAccessError>`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessError)
    pub fn disable_serial_console_access(&self) -> crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessFluentBuilder{
        crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessFluentBuilder::new(self.handle.clone())
    }
}
