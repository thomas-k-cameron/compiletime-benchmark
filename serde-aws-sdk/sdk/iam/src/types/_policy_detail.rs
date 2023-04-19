// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an IAM policy, including the policy document.</p>
/// <p>This data type is used as a response element in the <code>GetAccountAuthorizationDetails</code> operation.</p>
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
pub struct PolicyDetail {
    /// <p>The name of the policy.</p>
    #[doc(hidden)]
    pub policy_name: std::option::Option<std::string::String>,
    /// <p>The policy document.</p>
    #[doc(hidden)]
    pub policy_document: std::option::Option<std::string::String>,
}
impl PolicyDetail {
    /// <p>The name of the policy.</p>
    pub fn policy_name(&self) -> std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>The policy document.</p>
    pub fn policy_document(&self) -> std::option::Option<&str> {
        self.policy_document.as_deref()
    }
}
impl PolicyDetail {
    /// Creates a new builder-style object to manufacture [`PolicyDetail`](crate::types::PolicyDetail).
    pub fn builder() -> crate::types::builders::PolicyDetailBuilder {
        crate::types::builders::PolicyDetailBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PolicyDetail;
/// A builder for [`PolicyDetail`](crate::types::PolicyDetail).
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
pub struct PolicyDetailBuilder {
    pub(crate) policy_name: std::option::Option<std::string::String>,
    pub(crate) policy_document: std::option::Option<std::string::String>,
}
impl PolicyDetailBuilder {
    /// <p>The name of the policy.</p>
    pub fn policy_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_name = Some(input.into());
        self
    }
    /// <p>The name of the policy.</p>
    pub fn set_policy_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>The policy document.</p>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }
    /// <p>The policy document.</p>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// Consumes the builder and constructs a [`PolicyDetail`](crate::types::PolicyDetail).
    pub fn build(self) -> crate::types::PolicyDetail {
        crate::types::PolicyDetail {
            policy_name: self.policy_name,
            policy_document: self.policy_document,
        }
    }
}
