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
pub struct ModifyVerifiedAccessGroupPolicyInput {
    /// <p>The ID of the Amazon Web Services Verified Access group.</p>
    #[doc(hidden)]
    pub verified_access_group_id: std::option::Option<std::string::String>,
    /// <p>The status of the Verified Access policy.</p>
    #[doc(hidden)]
    pub policy_enabled: std::option::Option<bool>,
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    #[doc(hidden)]
    pub policy_document: std::option::Option<std::string::String>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl ModifyVerifiedAccessGroupPolicyInput {
    /// <p>The ID of the Amazon Web Services Verified Access group.</p>
    pub fn verified_access_group_id(&self) -> std::option::Option<&str> {
        self.verified_access_group_id.as_deref()
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn policy_enabled(&self) -> std::option::Option<bool> {
        self.policy_enabled
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn policy_document(&self) -> std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl ModifyVerifiedAccessGroupPolicyInput {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessGroupPolicyInput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput).
    pub fn builder() -> crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyInputBuilder{
        crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput;
/// A builder for [`ModifyVerifiedAccessGroupPolicyInput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput).
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
pub struct ModifyVerifiedAccessGroupPolicyInputBuilder {
    pub(crate) verified_access_group_id: std::option::Option<std::string::String>,
    pub(crate) policy_enabled: std::option::Option<bool>,
    pub(crate) policy_document: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl ModifyVerifiedAccessGroupPolicyInputBuilder {
    /// <p>The ID of the Amazon Web Services Verified Access group.</p>
    pub fn verified_access_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.verified_access_group_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Verified Access group.</p>
    pub fn set_verified_access_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.verified_access_group_id = input;
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn policy_enabled(mut self, input: bool) -> Self {
        self.policy_enabled = Some(input);
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn set_policy_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.policy_enabled = input;
        self
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Verified Access policy document.</p>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessGroupPolicyInput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyInput {
                verified_access_group_id: self.verified_access_group_id
                ,
                policy_enabled: self.policy_enabled
                ,
                policy_document: self.policy_document
                ,
                client_token: self.client_token
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}