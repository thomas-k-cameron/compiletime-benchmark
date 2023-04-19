// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether an Config rule is compliant based on account ID, region, compliance, and rule name.</p>
/// <p>A rule is compliant if all of the resources that the rule evaluated comply with it. It is noncompliant if any of these resources do not comply.</p>
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
pub struct AggregateComplianceByConfigRule {
    /// <p>The name of the Config rule.</p>
    #[doc(hidden)]
    pub config_rule_name: std::option::Option<std::string::String>,
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    #[doc(hidden)]
    pub compliance: std::option::Option<crate::types::Compliance>,
    /// <p>The 12-digit account ID of the source account.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[doc(hidden)]
    pub aws_region: std::option::Option<std::string::String>,
}
impl AggregateComplianceByConfigRule {
    /// <p>The name of the Config rule.</p>
    pub fn config_rule_name(&self) -> std::option::Option<&str> {
        self.config_rule_name.as_deref()
    }
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    pub fn compliance(&self) -> std::option::Option<&crate::types::Compliance> {
        self.compliance.as_ref()
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn aws_region(&self) -> std::option::Option<&str> {
        self.aws_region.as_deref()
    }
}
impl AggregateComplianceByConfigRule {
    /// Creates a new builder-style object to manufacture [`AggregateComplianceByConfigRule`](crate::types::AggregateComplianceByConfigRule).
    pub fn builder() -> crate::types::builders::AggregateComplianceByConfigRuleBuilder {
        crate::types::builders::AggregateComplianceByConfigRuleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AggregateComplianceByConfigRule;
/// A builder for [`AggregateComplianceByConfigRule`](crate::types::AggregateComplianceByConfigRule).
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
pub struct AggregateComplianceByConfigRuleBuilder {
    pub(crate) config_rule_name: std::option::Option<std::string::String>,
    pub(crate) compliance: std::option::Option<crate::types::Compliance>,
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) aws_region: std::option::Option<std::string::String>,
}
impl AggregateComplianceByConfigRuleBuilder {
    /// <p>The name of the Config rule.</p>
    pub fn config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.config_rule_name = Some(input.into());
        self
    }
    /// <p>The name of the Config rule.</p>
    pub fn set_config_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.config_rule_name = input;
        self
    }
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    pub fn compliance(mut self, input: crate::types::Compliance) -> Self {
        self.compliance = Some(input);
        self
    }
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    pub fn set_compliance(mut self, input: std::option::Option<crate::types::Compliance>) -> Self {
        self.compliance = input;
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn aws_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.aws_region = Some(input.into());
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn set_aws_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.aws_region = input;
        self
    }
    /// Consumes the builder and constructs a [`AggregateComplianceByConfigRule`](crate::types::AggregateComplianceByConfigRule).
    pub fn build(self) -> crate::types::AggregateComplianceByConfigRule {
        crate::types::AggregateComplianceByConfigRule {
            config_rule_name: self.config_rule_name,
            compliance: self.compliance,
            account_id: self.account_id,
            aws_region: self.aws_region,
        }
    }
}
