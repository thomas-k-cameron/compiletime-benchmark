// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCodeSigningConfig`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`code_signing_config_arn(impl Into<String>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::code_signing_config_arn) / [`set_code_signing_config_arn(Option<String>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::set_code_signing_config_arn): <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::set_description): <p>Descriptive name for this code signing configuration.</p>
    ///   - [`allowed_publishers(AllowedPublishers)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::allowed_publishers) / [`set_allowed_publishers(Option<AllowedPublishers>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::set_allowed_publishers): <p>Signing profiles for this code signing configuration.</p>
    ///   - [`code_signing_policies(CodeSigningPolicies)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::code_signing_policies) / [`set_code_signing_policies(Option<CodeSigningPolicies>)`](crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::set_code_signing_policies): <p>The code signing policy.</p>
    /// - On success, responds with [`UpdateCodeSigningConfigOutput`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigOutput) with field(s):
    ///   - [`code_signing_config(Option<CodeSigningConfig>)`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigOutput::code_signing_config): <p>The code signing configuration</p>
    /// - On failure, responds with [`SdkError<UpdateCodeSigningConfigError>`](crate::operation::update_code_signing_config::UpdateCodeSigningConfigError)
    pub fn update_code_signing_config(
        &self,
    ) -> crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder
    {
        crate::operation::update_code_signing_config::builders::UpdateCodeSigningConfigFluentBuilder::new(self.handle.clone())
    }
}
