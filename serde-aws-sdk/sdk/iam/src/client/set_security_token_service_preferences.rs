// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetSecurityTokenServicePreferences`](crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_endpoint_token_version(GlobalEndpointTokenVersion)`](crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder::global_endpoint_token_version) / [`set_global_endpoint_token_version(Option<GlobalEndpointTokenVersion>)`](crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder::set_global_endpoint_token_version): <p>The version of the global endpoint token. Version 1 tokens are valid only in Amazon Web Services Regions that are available by default. These tokens do not work in manually enabled Regions, such as Asia Pacific (Hong Kong). Version 2 tokens are valid in all Regions. However, version 2 tokens are longer and might affect systems where you temporarily store tokens.</p>  <p>For information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    /// - On success, responds with [`SetSecurityTokenServicePreferencesOutput`](crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput)
    /// - On failure, responds with [`SdkError<SetSecurityTokenServicePreferencesError>`](crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError)
    pub fn set_security_token_service_preferences(&self) -> crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder{
        crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesFluentBuilder::new(self.handle.clone())
    }
}
