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
pub struct CreateCodeSigningConfigInput {
    /// <p>Descriptive name for this code signing configuration.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>Signing profiles for this code signing configuration.</p>
    #[doc(hidden)]
    pub allowed_publishers: std::option::Option<crate::types::AllowedPublishers>,
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    #[doc(hidden)]
    pub code_signing_policies: std::option::Option<crate::types::CodeSigningPolicies>,
}
impl CreateCodeSigningConfigInput {
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(&self) -> std::option::Option<&crate::types::AllowedPublishers> {
        self.allowed_publishers.as_ref()
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    pub fn code_signing_policies(&self) -> std::option::Option<&crate::types::CodeSigningPolicies> {
        self.code_signing_policies.as_ref()
    }
}
impl CreateCodeSigningConfigInput {
    /// Creates a new builder-style object to manufacture [`CreateCodeSigningConfigInput`](crate::operation::create_code_signing_config::CreateCodeSigningConfigInput).
    pub fn builder(
    ) -> crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder
    {
        crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_code_signing_config::CreateCodeSigningConfigInput;
/// A builder for [`CreateCodeSigningConfigInput`](crate::operation::create_code_signing_config::CreateCodeSigningConfigInput).
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
pub struct CreateCodeSigningConfigInputBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) allowed_publishers: std::option::Option<crate::types::AllowedPublishers>,
    pub(crate) code_signing_policies: std::option::Option<crate::types::CodeSigningPolicies>,
}
impl CreateCodeSigningConfigInputBuilder {
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(mut self, input: crate::types::AllowedPublishers) -> Self {
        self.allowed_publishers = Some(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn set_allowed_publishers(
        mut self,
        input: std::option::Option<crate::types::AllowedPublishers>,
    ) -> Self {
        self.allowed_publishers = input;
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    pub fn code_signing_policies(mut self, input: crate::types::CodeSigningPolicies) -> Self {
        self.code_signing_policies = Some(input);
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    pub fn set_code_signing_policies(
        mut self,
        input: std::option::Option<crate::types::CodeSigningPolicies>,
    ) -> Self {
        self.code_signing_policies = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateCodeSigningConfigInput`](crate::operation::create_code_signing_config::CreateCodeSigningConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_code_signing_config::CreateCodeSigningConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_code_signing_config::CreateCodeSigningConfigInput {
                description: self.description,
                allowed_publishers: self.allowed_publishers,
                code_signing_policies: self.code_signing_policies,
            },
        )
    }
}
