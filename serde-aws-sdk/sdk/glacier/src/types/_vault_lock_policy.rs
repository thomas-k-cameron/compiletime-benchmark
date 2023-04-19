// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the vault lock policy.</p>
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
pub struct VaultLockPolicy {
    /// <p>The vault lock policy.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<std::string::String>,
}
impl VaultLockPolicy {
    /// <p>The vault lock policy.</p>
    pub fn policy(&self) -> std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl VaultLockPolicy {
    /// Creates a new builder-style object to manufacture [`VaultLockPolicy`](crate::types::VaultLockPolicy).
    pub fn builder() -> crate::types::builders::VaultLockPolicyBuilder {
        crate::types::builders::VaultLockPolicyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VaultLockPolicy;
/// A builder for [`VaultLockPolicy`](crate::types::VaultLockPolicy).
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
pub struct VaultLockPolicyBuilder {
    pub(crate) policy: std::option::Option<std::string::String>,
}
impl VaultLockPolicyBuilder {
    /// <p>The vault lock policy.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy = Some(input.into());
        self
    }
    /// <p>The vault lock policy.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`VaultLockPolicy`](crate::types::VaultLockPolicy).
    pub fn build(self) -> crate::types::VaultLockPolicy {
        crate::types::VaultLockPolicy {
            policy: self.policy,
        }
    }
}