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
pub struct DeleteUserPolicyInput {
    /// <p>The name (friendly name, not ARN) identifying the user that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The name identifying the policy document to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub policy_name: std::option::Option<std::string::String>,
}
impl DeleteUserPolicyInput {
    /// <p>The name (friendly name, not ARN) identifying the user that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The name identifying the policy document to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(&self) -> std::option::Option<&str> {
        self.policy_name.as_deref()
    }
}
impl DeleteUserPolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteUserPolicyInput`](crate::operation::delete_user_policy::DeleteUserPolicyInput).
    pub fn builder() -> crate::operation::delete_user_policy::builders::DeleteUserPolicyInputBuilder
    {
        crate::operation::delete_user_policy::builders::DeleteUserPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_user_policy::DeleteUserPolicyInput;
/// A builder for [`DeleteUserPolicyInput`](crate::operation::delete_user_policy::DeleteUserPolicyInput).
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
pub struct DeleteUserPolicyInputBuilder {
    pub(crate) user_name: std::option::Option<std::string::String>,
    pub(crate) policy_name: std::option::Option<std::string::String>,
}
impl DeleteUserPolicyInputBuilder {
    /// <p>The name (friendly name, not ARN) identifying the user that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_name = Some(input.into());
        self
    }
    /// <p>The name (friendly name, not ARN) identifying the user that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The name identifying the policy document to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_name = Some(input.into());
        self
    }
    /// <p>The name identifying the policy document to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_policy_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteUserPolicyInput`](crate::operation::delete_user_policy::DeleteUserPolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_user_policy::DeleteUserPolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_user_policy::DeleteUserPolicyInput {
                user_name: self.user_name,
                policy_name: self.policy_name,
            },
        )
    }
}
