// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a transit gateway policy table entry</p>
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
pub struct TransitGatewayPolicyTableEntry {
    /// <p>The rule number for the transit gateway policy table entry.</p>
    #[doc(hidden)]
    pub policy_rule_number: std::option::Option<std::string::String>,
    /// <p>The policy rule associated with the transit gateway policy table.</p>
    #[doc(hidden)]
    pub policy_rule: std::option::Option<crate::types::TransitGatewayPolicyRule>,
    /// <p>The ID of the target route table.</p>
    #[doc(hidden)]
    pub target_route_table_id: std::option::Option<std::string::String>,
}
impl TransitGatewayPolicyTableEntry {
    /// <p>The rule number for the transit gateway policy table entry.</p>
    pub fn policy_rule_number(&self) -> std::option::Option<&str> {
        self.policy_rule_number.as_deref()
    }
    /// <p>The policy rule associated with the transit gateway policy table.</p>
    pub fn policy_rule(&self) -> std::option::Option<&crate::types::TransitGatewayPolicyRule> {
        self.policy_rule.as_ref()
    }
    /// <p>The ID of the target route table.</p>
    pub fn target_route_table_id(&self) -> std::option::Option<&str> {
        self.target_route_table_id.as_deref()
    }
}
impl TransitGatewayPolicyTableEntry {
    /// Creates a new builder-style object to manufacture [`TransitGatewayPolicyTableEntry`](crate::types::TransitGatewayPolicyTableEntry).
    pub fn builder() -> crate::types::builders::TransitGatewayPolicyTableEntryBuilder {
        crate::types::builders::TransitGatewayPolicyTableEntryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayPolicyTableEntry;
/// A builder for [`TransitGatewayPolicyTableEntry`](crate::types::TransitGatewayPolicyTableEntry).
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
pub struct TransitGatewayPolicyTableEntryBuilder {
    pub(crate) policy_rule_number: std::option::Option<std::string::String>,
    pub(crate) policy_rule: std::option::Option<crate::types::TransitGatewayPolicyRule>,
    pub(crate) target_route_table_id: std::option::Option<std::string::String>,
}
impl TransitGatewayPolicyTableEntryBuilder {
    /// <p>The rule number for the transit gateway policy table entry.</p>
    pub fn policy_rule_number(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_rule_number = Some(input.into());
        self
    }
    /// <p>The rule number for the transit gateway policy table entry.</p>
    pub fn set_policy_rule_number(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.policy_rule_number = input;
        self
    }
    /// <p>The policy rule associated with the transit gateway policy table.</p>
    pub fn policy_rule(mut self, input: crate::types::TransitGatewayPolicyRule) -> Self {
        self.policy_rule = Some(input);
        self
    }
    /// <p>The policy rule associated with the transit gateway policy table.</p>
    pub fn set_policy_rule(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayPolicyRule>,
    ) -> Self {
        self.policy_rule = input;
        self
    }
    /// <p>The ID of the target route table.</p>
    pub fn target_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.target_route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the target route table.</p>
    pub fn set_target_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.target_route_table_id = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayPolicyTableEntry`](crate::types::TransitGatewayPolicyTableEntry).
    pub fn build(self) -> crate::types::TransitGatewayPolicyTableEntry {
        crate::types::TransitGatewayPolicyTableEntry {
            policy_rule_number: self.policy_rule_number,
            policy_rule: self.policy_rule,
            target_route_table_id: self.target_route_table_id,
        }
    }
}
