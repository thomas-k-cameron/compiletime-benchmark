// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
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
pub struct OrganizationCustomRuleMetadata {
    /// <p>The description that you provide for your organization Config rule.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The lambda function ARN.</p>
    #[doc(hidden)]
    pub lambda_function_arn: std::option::Option<std::string::String>,
    /// <p>The type of notification that triggers Config to run an evaluation for a rule. You can specify the following notification types:</p>
    /// <ul>
    /// <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers a configuration item as a result of a resource change.</p> </li>
    /// <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers an oversized configuration item. Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li>
    /// <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub organization_config_rule_trigger_types:
        std::option::Option<std::vec::Vec<crate::types::OrganizationConfigRuleTriggerType>>,
    /// <p>A string, in JSON format, that is passed to your organization Config rule Lambda function.</p>
    #[doc(hidden)]
    pub input_parameters: std::option::Option<std::string::String>,
    /// <p>The maximum frequency with which Config runs evaluations for a rule. Your custom rule is triggered when Config delivers the configuration snapshot. For more information, see <code>ConfigSnapshotDeliveryProperties</code>.</p> <note>
    /// <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p>
    /// </note>
    #[doc(hidden)]
    pub maximum_execution_frequency: std::option::Option<crate::types::MaximumExecutionFrequency>,
    /// <p>The type of the Amazon Web Services resource that was evaluated.</p>
    #[doc(hidden)]
    pub resource_types_scope: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The ID of the Amazon Web Services resource that was evaluated.</p>
    #[doc(hidden)]
    pub resource_id_scope: std::option::Option<std::string::String>,
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    #[doc(hidden)]
    pub tag_key_scope: std::option::Option<std::string::String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). </p>
    #[doc(hidden)]
    pub tag_value_scope: std::option::Option<std::string::String>,
}
impl OrganizationCustomRuleMetadata {
    /// <p>The description that you provide for your organization Config rule.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The lambda function ARN.</p>
    pub fn lambda_function_arn(&self) -> std::option::Option<&str> {
        self.lambda_function_arn.as_deref()
    }
    /// <p>The type of notification that triggers Config to run an evaluation for a rule. You can specify the following notification types:</p>
    /// <ul>
    /// <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers a configuration item as a result of a resource change.</p> </li>
    /// <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers an oversized configuration item. Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li>
    /// <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li>
    /// </ul>
    pub fn organization_config_rule_trigger_types(
        &self,
    ) -> std::option::Option<&[crate::types::OrganizationConfigRuleTriggerType]> {
        self.organization_config_rule_trigger_types.as_deref()
    }
    /// <p>A string, in JSON format, that is passed to your organization Config rule Lambda function.</p>
    pub fn input_parameters(&self) -> std::option::Option<&str> {
        self.input_parameters.as_deref()
    }
    /// <p>The maximum frequency with which Config runs evaluations for a rule. Your custom rule is triggered when Config delivers the configuration snapshot. For more information, see <code>ConfigSnapshotDeliveryProperties</code>.</p> <note>
    /// <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p>
    /// </note>
    pub fn maximum_execution_frequency(
        &self,
    ) -> std::option::Option<&crate::types::MaximumExecutionFrequency> {
        self.maximum_execution_frequency.as_ref()
    }
    /// <p>The type of the Amazon Web Services resource that was evaluated.</p>
    pub fn resource_types_scope(&self) -> std::option::Option<&[std::string::String]> {
        self.resource_types_scope.as_deref()
    }
    /// <p>The ID of the Amazon Web Services resource that was evaluated.</p>
    pub fn resource_id_scope(&self) -> std::option::Option<&str> {
        self.resource_id_scope.as_deref()
    }
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    pub fn tag_key_scope(&self) -> std::option::Option<&str> {
        self.tag_key_scope.as_deref()
    }
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). </p>
    pub fn tag_value_scope(&self) -> std::option::Option<&str> {
        self.tag_value_scope.as_deref()
    }
}
impl OrganizationCustomRuleMetadata {
    /// Creates a new builder-style object to manufacture [`OrganizationCustomRuleMetadata`](crate::types::OrganizationCustomRuleMetadata).
    pub fn builder() -> crate::types::builders::OrganizationCustomRuleMetadataBuilder {
        crate::types::builders::OrganizationCustomRuleMetadataBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::OrganizationCustomRuleMetadata;
/// A builder for [`OrganizationCustomRuleMetadata`](crate::types::OrganizationCustomRuleMetadata).
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
pub struct OrganizationCustomRuleMetadataBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) lambda_function_arn: std::option::Option<std::string::String>,
    pub(crate) organization_config_rule_trigger_types:
        std::option::Option<std::vec::Vec<crate::types::OrganizationConfigRuleTriggerType>>,
    pub(crate) input_parameters: std::option::Option<std::string::String>,
    pub(crate) maximum_execution_frequency:
        std::option::Option<crate::types::MaximumExecutionFrequency>,
    pub(crate) resource_types_scope: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) resource_id_scope: std::option::Option<std::string::String>,
    pub(crate) tag_key_scope: std::option::Option<std::string::String>,
    pub(crate) tag_value_scope: std::option::Option<std::string::String>,
}
impl OrganizationCustomRuleMetadataBuilder {
    /// <p>The description that you provide for your organization Config rule.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description that you provide for your organization Config rule.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The lambda function ARN.</p>
    pub fn lambda_function_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.lambda_function_arn = Some(input.into());
        self
    }
    /// <p>The lambda function ARN.</p>
    pub fn set_lambda_function_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.lambda_function_arn = input;
        self
    }
    /// Appends an item to `organization_config_rule_trigger_types`.
    ///
    /// To override the contents of this collection use [`set_organization_config_rule_trigger_types`](Self::set_organization_config_rule_trigger_types).
    ///
    /// <p>The type of notification that triggers Config to run an evaluation for a rule. You can specify the following notification types:</p>
    /// <ul>
    /// <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers a configuration item as a result of a resource change.</p> </li>
    /// <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers an oversized configuration item. Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li>
    /// <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li>
    /// </ul>
    pub fn organization_config_rule_trigger_types(
        mut self,
        input: crate::types::OrganizationConfigRuleTriggerType,
    ) -> Self {
        let mut v = self
            .organization_config_rule_trigger_types
            .unwrap_or_default();
        v.push(input);
        self.organization_config_rule_trigger_types = Some(v);
        self
    }
    /// <p>The type of notification that triggers Config to run an evaluation for a rule. You can specify the following notification types:</p>
    /// <ul>
    /// <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers a configuration item as a result of a resource change.</p> </li>
    /// <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when Config delivers an oversized configuration item. Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li>
    /// <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li>
    /// </ul>
    pub fn set_organization_config_rule_trigger_types(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::OrganizationConfigRuleTriggerType>>,
    ) -> Self {
        self.organization_config_rule_trigger_types = input;
        self
    }
    /// <p>A string, in JSON format, that is passed to your organization Config rule Lambda function.</p>
    pub fn input_parameters(mut self, input: impl Into<std::string::String>) -> Self {
        self.input_parameters = Some(input.into());
        self
    }
    /// <p>A string, in JSON format, that is passed to your organization Config rule Lambda function.</p>
    pub fn set_input_parameters(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.input_parameters = input;
        self
    }
    /// <p>The maximum frequency with which Config runs evaluations for a rule. Your custom rule is triggered when Config delivers the configuration snapshot. For more information, see <code>ConfigSnapshotDeliveryProperties</code>.</p> <note>
    /// <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p>
    /// </note>
    pub fn maximum_execution_frequency(
        mut self,
        input: crate::types::MaximumExecutionFrequency,
    ) -> Self {
        self.maximum_execution_frequency = Some(input);
        self
    }
    /// <p>The maximum frequency with which Config runs evaluations for a rule. Your custom rule is triggered when Config delivers the configuration snapshot. For more information, see <code>ConfigSnapshotDeliveryProperties</code>.</p> <note>
    /// <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p>
    /// </note>
    pub fn set_maximum_execution_frequency(
        mut self,
        input: std::option::Option<crate::types::MaximumExecutionFrequency>,
    ) -> Self {
        self.maximum_execution_frequency = input;
        self
    }
    /// Appends an item to `resource_types_scope`.
    ///
    /// To override the contents of this collection use [`set_resource_types_scope`](Self::set_resource_types_scope).
    ///
    /// <p>The type of the Amazon Web Services resource that was evaluated.</p>
    pub fn resource_types_scope(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resource_types_scope.unwrap_or_default();
        v.push(input.into());
        self.resource_types_scope = Some(v);
        self
    }
    /// <p>The type of the Amazon Web Services resource that was evaluated.</p>
    pub fn set_resource_types_scope(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resource_types_scope = input;
        self
    }
    /// <p>The ID of the Amazon Web Services resource that was evaluated.</p>
    pub fn resource_id_scope(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_id_scope = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services resource that was evaluated.</p>
    pub fn set_resource_id_scope(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.resource_id_scope = input;
        self
    }
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    pub fn tag_key_scope(mut self, input: impl Into<std::string::String>) -> Self {
        self.tag_key_scope = Some(input.into());
        self
    }
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    pub fn set_tag_key_scope(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.tag_key_scope = input;
        self
    }
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). </p>
    pub fn tag_value_scope(mut self, input: impl Into<std::string::String>) -> Self {
        self.tag_value_scope = Some(input.into());
        self
    }
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). </p>
    pub fn set_tag_value_scope(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.tag_value_scope = input;
        self
    }
    /// Consumes the builder and constructs a [`OrganizationCustomRuleMetadata`](crate::types::OrganizationCustomRuleMetadata).
    pub fn build(self) -> crate::types::OrganizationCustomRuleMetadata {
        crate::types::OrganizationCustomRuleMetadata {
            description: self.description,
            lambda_function_arn: self.lambda_function_arn,
            organization_config_rule_trigger_types: self.organization_config_rule_trigger_types,
            input_parameters: self.input_parameters,
            maximum_execution_frequency: self.maximum_execution_frequency,
            resource_types_scope: self.resource_types_scope,
            resource_id_scope: self.resource_id_scope,
            tag_key_scope: self.tag_key_scope,
            tag_value_scope: self.tag_value_scope,
        }
    }
}
