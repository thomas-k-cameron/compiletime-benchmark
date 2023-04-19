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
pub struct StartRemediationExecutionInput {
    /// <p>The list of names of Config rules that you want to run remediation execution for.</p>
    #[doc(hidden)]
    pub config_rule_name: std::option::Option<std::string::String>,
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    #[doc(hidden)]
    pub resource_keys: std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
}
impl StartRemediationExecutionInput {
    /// <p>The list of names of Config rules that you want to run remediation execution for.</p>
    pub fn config_rule_name(&self) -> std::option::Option<&str> {
        self.config_rule_name.as_deref()
    }
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    pub fn resource_keys(&self) -> std::option::Option<&[crate::types::ResourceKey]> {
        self.resource_keys.as_deref()
    }
}
impl StartRemediationExecutionInput {
    /// Creates a new builder-style object to manufacture [`StartRemediationExecutionInput`](crate::operation::start_remediation_execution::StartRemediationExecutionInput).
    pub fn builder() -> crate::operation::start_remediation_execution::builders::StartRemediationExecutionInputBuilder{
        crate::operation::start_remediation_execution::builders::StartRemediationExecutionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::start_remediation_execution::StartRemediationExecutionInput;
/// A builder for [`StartRemediationExecutionInput`](crate::operation::start_remediation_execution::StartRemediationExecutionInput).
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
pub struct StartRemediationExecutionInputBuilder {
    pub(crate) config_rule_name: std::option::Option<std::string::String>,
    pub(crate) resource_keys: std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
}
impl StartRemediationExecutionInputBuilder {
    /// <p>The list of names of Config rules that you want to run remediation execution for.</p>
    pub fn config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.config_rule_name = Some(input.into());
        self
    }
    /// <p>The list of names of Config rules that you want to run remediation execution for.</p>
    pub fn set_config_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.config_rule_name = input;
        self
    }
    /// Appends an item to `resource_keys`.
    ///
    /// To override the contents of this collection use [`set_resource_keys`](Self::set_resource_keys).
    ///
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    pub fn resource_keys(mut self, input: crate::types::ResourceKey) -> Self {
        let mut v = self.resource_keys.unwrap_or_default();
        v.push(input);
        self.resource_keys = Some(v);
        self
    }
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    pub fn set_resource_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
    ) -> Self {
        self.resource_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`StartRemediationExecutionInput`](crate::operation::start_remediation_execution::StartRemediationExecutionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::start_remediation_execution::StartRemediationExecutionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::start_remediation_execution::StartRemediationExecutionInput {
                config_rule_name: self.config_rule_name,
                resource_keys: self.resource_keys,
            },
        )
    }
}
