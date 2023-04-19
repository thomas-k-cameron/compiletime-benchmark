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
pub struct DetachRolePolicyInput {
    /// <p>The name (friendly name, not ARN) of the IAM role to detach the policy from.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM policy you want to detach.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub policy_arn: std::option::Option<std::string::String>,
}
impl DetachRolePolicyInput {
    /// <p>The name (friendly name, not ARN) of the IAM role to detach the policy from.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM policy you want to detach.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_arn(&self) -> std::option::Option<&str> {
        self.policy_arn.as_deref()
    }
}
impl DetachRolePolicyInput {
    /// Creates a new builder-style object to manufacture [`DetachRolePolicyInput`](crate::operation::detach_role_policy::DetachRolePolicyInput).
    pub fn builder() -> crate::operation::detach_role_policy::builders::DetachRolePolicyInputBuilder
    {
        crate::operation::detach_role_policy::builders::DetachRolePolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::detach_role_policy::DetachRolePolicyInput;
/// A builder for [`DetachRolePolicyInput`](crate::operation::detach_role_policy::DetachRolePolicyInput).
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
pub struct DetachRolePolicyInputBuilder {
    pub(crate) role_name: std::option::Option<std::string::String>,
    pub(crate) policy_arn: std::option::Option<std::string::String>,
}
impl DetachRolePolicyInputBuilder {
    /// <p>The name (friendly name, not ARN) of the IAM role to detach the policy from.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_name = Some(input.into());
        self
    }
    /// <p>The name (friendly name, not ARN) of the IAM role to detach the policy from.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM policy you want to detach.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM policy you want to detach.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_policy_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DetachRolePolicyInput`](crate::operation::detach_role_policy::DetachRolePolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::detach_role_policy::DetachRolePolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::detach_role_policy::DetachRolePolicyInput {
                role_name: self.role_name,
                policy_arn: self.policy_arn,
            },
        )
    }
}