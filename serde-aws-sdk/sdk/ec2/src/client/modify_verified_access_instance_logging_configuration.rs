// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessInstanceLoggingConfiguration`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_instance_id(impl Into<String>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::verified_access_instance_id) / [`set_verified_access_instance_id(Option<String>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::set_verified_access_instance_id): <p>The ID of the Amazon Web Services Verified Access instance.</p>
    ///   - [`access_logs(VerifiedAccessLogOptions)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::access_logs) / [`set_access_logs(Option<VerifiedAccessLogOptions>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::set_access_logs): <p>The configuration options for Amazon Web Services Verified Access instances.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    /// - On success, responds with [`ModifyVerifiedAccessInstanceLoggingConfigurationOutput`](crate::operation::modify_verified_access_instance_logging_configuration::ModifyVerifiedAccessInstanceLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<VerifiedAccessInstanceLoggingConfiguration>)`](crate::operation::modify_verified_access_instance_logging_configuration::ModifyVerifiedAccessInstanceLoggingConfigurationOutput::logging_configuration): <p>The logging configuration for Amazon Web Services Verified Access instance.</p>
    /// - On failure, responds with [`SdkError<ModifyVerifiedAccessInstanceLoggingConfigurationError>`](crate::operation::modify_verified_access_instance_logging_configuration::ModifyVerifiedAccessInstanceLoggingConfigurationError)
    pub fn modify_verified_access_instance_logging_configuration(&self) -> crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder{
        crate::operation::modify_verified_access_instance_logging_configuration::builders::ModifyVerifiedAccessInstanceLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
