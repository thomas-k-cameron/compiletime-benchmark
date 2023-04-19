// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input values for <code>InitiateVaultLock</code>.</p>
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
pub struct InitiateVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the vault.</p>
    #[doc(hidden)]
    pub vault_name: std::option::Option<std::string::String>,
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<crate::types::VaultLockPolicy>,
}
impl InitiateVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(&self) -> std::option::Option<&str> {
        self.vault_name.as_deref()
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn policy(&self) -> std::option::Option<&crate::types::VaultLockPolicy> {
        self.policy.as_ref()
    }
}
impl InitiateVaultLockInput {
    /// Creates a new builder-style object to manufacture [`InitiateVaultLockInput`](crate::operation::initiate_vault_lock::InitiateVaultLockInput).
    pub fn builder(
    ) -> crate::operation::initiate_vault_lock::builders::InitiateVaultLockInputBuilder {
        crate::operation::initiate_vault_lock::builders::InitiateVaultLockInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::initiate_vault_lock::InitiateVaultLockInput;
/// A builder for [`InitiateVaultLockInput`](crate::operation::initiate_vault_lock::InitiateVaultLockInput).
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
pub struct InitiateVaultLockInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) vault_name: std::option::Option<std::string::String>,
    pub(crate) policy: std::option::Option<crate::types::VaultLockPolicy>,
}
impl InitiateVaultLockInputBuilder {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.vault_name = Some(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vault_name = input;
        self
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn policy(mut self, input: crate::types::VaultLockPolicy) -> Self {
        self.policy = Some(input);
        self
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn set_policy(mut self, input: std::option::Option<crate::types::VaultLockPolicy>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`InitiateVaultLockInput`](crate::operation::initiate_vault_lock::InitiateVaultLockInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::initiate_vault_lock::InitiateVaultLockInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::initiate_vault_lock::InitiateVaultLockInput {
                account_id: self.account_id,
                vault_name: self.vault_name,
                policy: self.policy,
            },
        )
    }
}
